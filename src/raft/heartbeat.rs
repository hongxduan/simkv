//! Heartbeat implement
//!
//! author: Duan HongXing
//! date: 22 Apr, 2025
//!

use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use serde::{Deserialize, Serialize};
use tokio::{io::AsyncWriteExt, net::TcpStream, runtime::Runtime, time::Instant};

use lazy_static::lazy_static;

use super::raft::{GLOBAL_NODE_INFO, GLOBAL_RAFT_DATA, GLOBAL_RAFT_STATE, Node, RaftRole};

const HB_LOWER: u16 = 200; // random lower
const HB_UPPER: u16 = 400; // random upper

#[derive(Debug, Serialize, Deserialize)]
pub struct HeartbeatData {
    lnode_id: String, // Send Leader node id to followers
}

#[derive(Debug)]
pub struct Heartbeat {
    //sockets: Arc<Mutex<HashMap<String, TcpStream>>>,
    //sockets: HashMap<String,  TcpStream>
    sockets: HashMap<String, Arc<Mutex<TcpStream>>>,
}

pub struct CachedSocket {
    pub shared: Arc<Shared>,
}
pub struct Shared {
    pub sockets: Mutex<HashMap<String, TcpStream>>,
}

lazy_static! {
    static ref CACHED_HB_SOCKETS: CachedSocket = CachedSocket {
        shared: Arc::new(Shared {
            sockets: Mutex::new(HashMap::new())
        })
    };
}

impl Heartbeat {
    pub fn new() -> Self {
        Heartbeat {
            //sockets: Arc::new(Mutex::new(HashMap::new())),
            sockets: HashMap::new(),
        }
    }

    ///
    /// Send heartbeat request
    ///
    pub async fn send(&mut self) {
        //let mut sockets: HashMap<String, TcpStream> = HashMap::new();
        /*let cached_sockets = CachedSocket {
            shared: Arc::new(Shared {
                sockets: Mutex::new(HashMap::new())
            })
        };*/
        //let cached_sockets: HashMap<String, Arc<Mutex<TcpStream>>> = HashMap::new();
        
        let mut interval_timer =
            tokio::time::interval(chrono::Duration::milliseconds(300).to_std().unwrap());

        //let mut sockets = self.sockets.lock().unwrap();
        loop {
            interval_timer.tick().await;

            // Check Raft Role, If Follower, then break
            let raft_state_lock = GLOBAL_RAFT_STATE.read().unwrap();
            let _ = match raft_state_lock.role {
                RaftRole::Follower => break,
                RaftRole::Leader => {}
            };
            drop(raft_state_lock);

            // Get Raft Leader(current) node
            let read_lock = GLOBAL_NODE_INFO.read().unwrap();
            let lnode_id = read_lock.id.clone();
            //drop(read_lock);

            // Heartbeat data, including the Leader node id to Follower
            let heartbeat: HeartbeatData = HeartbeatData { lnode_id };
            let serde_result = serde_json::to_string(&heartbeat);

            let raft_data_lock = GLOBAL_RAFT_DATA.read().unwrap();
            let nodes = &raft_data_lock.raft_data.nodes;
            match serde_result {
                Ok(content) => {
                    for (key, node) in nodes.clone() {
                        let buf = content.clone().as_bytes().to_vec();
                        //tokio::task::spawn_blocking( self.do_send1(buf, node));
                        self.do_send1(buf,node).await;
                    }
                }
                Err(e) => {
                    println!("heartbeat::do_send- {:?}", e);
                }
            }
            drop(raft_data_lock);
        }
    }

    async fn prepare_socket(host: String, port: u16) -> Option<TcpStream> {
        let socket = TcpStream::connect(format!("{}:{}", host.clone(), port)).await;
        match socket {
            Ok(socket) => Some(socket),
            Err(e) => None,
        }
    }

    async fn do_send1(&mut self, hb_msg: Vec<u8>, node: Node) {
        //let mut cached_socket_lock = CACHED_HB_SOCKET.;
        //let mut sockets = cached_sockets.shared.sockets.lock().unwrap();

        let mut cached_socket: Option<&mut TcpStream> = None;

        if self.sockets.contains_key(&node.id.clone()) {
            let mut state  = self.sockets.get(&node.id.clone()).unwrap();
            //let socket = tmp.socket;
            //cached_socket = Some(CachedSocket { socket });
            //cached_socket = Some(&mut tmp);
            let mut sock = state.lock().unwrap();
            sock.write_all(&hb_msg);
        } else {
            let rt = Runtime::new().unwrap();
            let prepared_socket = rt.block_on(Self::prepare_socket(node.host.clone(), node.port));

            match prepared_socket {
                Some(mut s) => {
                    s.write_all(&hb_msg);
                    self.sockets.insert(node.id.clone(),Arc::new(Mutex::new(s)));
                    //cached_socket = Some(&mut s);
                }
                None => {
                    println!(
                        "heartbeat::do_send- connnect to {} failed",
                        node.host.clone()
                    );
                }
            }
        }

        // Write heartbeat message
        match cached_socket {
            Some(socket) => {
                socket.write_all(&hb_msg);
            }
            None => {}
        }
    }

    ///
    /// Send heartbeat to each node in the Raft cluster
    /// The socket is cached for each node
    /// If socket is cached, then use the socket
    /// Or else, create new socket for the node
    ///
    /*async fn do_send(hb_msg: Vec<u8>) {
        let raft_lock = GLOBAL_RAFT_DATA.read().unwrap();
        let mut cached_socket_lock = CACHED_HB_SOCKETS.write().unwrap();
        let nodes = &raft_lock.raft_data.nodes;
        for (_, node) in nodes {
            // Check if socket cached or not
            // If not, the create new socket and cache it
            let mut socket: Option<TcpStream> = None;
            if cached_socket_lock.contains_key(&node.id.clone()) {
                socket = Some(cached_socket_lock.get(&node.id.clone()).unwrap());
            } else {
                let connect_result =
                    TcpStream::connect(format!("{}:{}", node.host.clone(), node.port)).await;
                socket = match connect_result {
                    Ok(s) => {
                        // Store the new socket
                        cached_socket_lock.insert(node.id.clone(), s);
                        let temp = cached_socket_lock.get(&node.id.clone()).unwrap();
                        Some(s)
                    }
                    Err(e) => {
                        println!(
                            "heartbeat::do_send- connect to {} failed, {}",
                            node.host.clone(),
                            e
                        );
                        None
                    }
                };
            }

            // Write heartbeat message
            match socket {
                Some(socket) => {
                    socket.write_all(&hb_msg);
                }
                None => {}
            }
        }
    }*/

    ///
    /// Receive heartbeat request
    ///
    pub fn receive() {
        // Update
        let mut write_lock = GLOBAL_RAFT_STATE.write().unwrap();
        write_lock.updated = Instant::now();
        drop(write_lock);
    }
}
