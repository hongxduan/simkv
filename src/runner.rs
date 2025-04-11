//! Server Runner
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!
use std::io::Error;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{Semaphore, broadcast, mpsc},
    time::{self, Duration},
};

use crate::{
    cmd::command::Command,
    db::db::{Db, DbDropGuard},
};
use simkv::shutdown::Shutdown;

#[derive(Debug)]
struct Listener {
    /// Server TCP listener
    listener: TcpListener,

    ///
    db_guard: DbDropGuard,

    /// Max allowed connections
    max_conns: Arc<Semaphore>,

    ///
    notify_shutdown: broadcast::Sender<()>,

    //
    shutdown_complete_tx: mpsc::Sender<()>,
}

#[derive(Debug)]
struct Handler {
    db: Db,
    socket: TcpStream,
    shutdown: Shutdown,
    _shutdown_complete: mpsc::Sender<()>,
}

pub async fn run(listener: TcpListener, shutdown: impl Future) {
    // Initialize listener
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel(1);
    let mut listener = Listener {
        listener,
        db_guard: DbDropGuard::new(),
        max_conns: Arc::new(Semaphore::new(100)), // will read from config
        notify_shutdown: notify_shutdown,
        shutdown_complete_tx,
    };

    tokio::select! {
        res = listener.run()=>{
            if let Err(err) = res{
                // log
                println!("runner::run- {}", err);
            }
        },
        _ = shutdown=>{
            // log
            println!("shutting down");
        }
    }

    let Listener {
        shutdown_complete_tx,
        notify_shutdown,
        ..
    } = listener;

    drop(notify_shutdown);
    drop(shutdown_complete_tx);

    let _ = shutdown_complete_rx.recv().await;
}

impl Listener {
    async fn run(&mut self) -> Result<(), Error> {
        loop {
            let permit = self.max_conns.clone().acquire_owned().await.unwrap();
            let socket = self.accept().await?;

            let mut handler = Handler {
                db: self.db_guard.db(),
                socket,
                shutdown: Shutdown::new(self.notify_shutdown.subscribe()),
                _shutdown_complete: self.shutdown_complete_tx.clone(),
            };
            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    println!("{}", err);
                }
                // Drop permit and return to Semaphore
                drop(permit);
            });
        }
    }

    async fn accept(&mut self) -> Result<TcpStream, Error> {
        let mut backoff = 1;

        // Accept loop
        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(err) => {
                    println!("runner::accept- {}", err);
                    if backoff > 64 {
                        return Err(err);
                    }
                }
            }
            time::sleep(Duration::from_secs(backoff)).await;

            backoff *= 2;
        }
    }
}

impl Handler {
    async fn run(&mut self) -> Result<(), Error> {
        println!("runner::Handler::run");
        let mut lbuf: [u8; 4] = [0, 0, 0, 0];
        while !self.shutdown.is_shutdown() {
            // Read message lenght bytes
            let _n = match self.socket.read(&mut lbuf).await {
                // socket closed
                Ok(0) => return Ok(()),
                Ok(n) => n,
                Err(e) => {
                    eprintln!("failed to read from socket; err = {:?}", e);
                    return Err(e.into());
                }
            };

            // Why this happen???
            if lbuf == [0, 0, 0, 0] {
                continue;
            }

            //
            // Read one single command byte length
            //
            let mlen = u32::from_be_bytes(lbuf) as usize;
            let mut buf = vec![0; mlen.try_into().unwrap()];

            //
            // Read message content based on the length readed above
            //
            let mut n: usize = 0;
            while n < mlen {
                n += match self.socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => return Ok(()),
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return Err(e);
                    }
                };
            }

            //println!("mlen: {}, {:?}", mlen, String::from_utf8(buf.clone()));

            //
            // Parse command
            //
            let command = Command::parse_command(&buf);

            //
            // Execute command
            //
            let result = command.execute(&self.db);

            // Write the data back
            if let Err(e) = self.socket.write_all(&result).await {
                eprintln!("failed to write to socket; err = {:?}", e);
                return Err(e);
            }
            //let _ = socket.write_all(&result).await;
            //let _ = socket.flush().await;
        }
        Ok(())
    }
}
