//! Raft Implement
//!
//! author: Duan HongXing
//! date: 4 Apr, 2025
//!
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::{MAIN_SEPARATOR, Path},
    sync::{Arc, Mutex, RwLock},
};

use tokio::{net::TcpListener, time::Instant};

use crate::{raft::handler::Handler, server::config::Config};
use serde::{Deserialize, Serialize};

use lazy_static::lazy_static;

use super::heartbeat::Heartbeat;

const RAFT_DATA_FILE_NAME: &str = "__raft_data__.json";
const NODE_FILE_NAME: &str = "__node__.json";

///
/// Node info
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub host: String,
    pub port: u16,
}

///
/// On which node the bucket stored
///
pub struct BucketNode {
    primary: String,       // the primary node id, only primary can write
    replicas: Vec<String>, // the replicas, 1 replica for now
}

///
/// Raft data that need sync by leader to all followers
///
pub struct RaftData {
    leader_node_id: String,
    nodes: HashMap<String, Node>,
    buckets: HashMap<u16, BucketNode>, // bucket_id on which node_id
    ts: Instant,                       // when updated
}

///
/// Raft state at runtime
///
pub struct RaftState {
    pub role: RaftRole,
    pub last_hb: Instant, // Last heartbeat instant received from Leader
}

#[derive(Debug, Clone)]
pub enum RaftRole {
    Leader,
    Follower,
}

#[derive(Debug, Clone)]
pub struct Raft {
    pub shared: Arc<Shared>,
}

#[derive(Debug)]
pub struct State {
    pub role: RaftRole,
    pub last_hb: Instant, // Last heartbeat instant received from Leader
}

#[derive(Debug)]
pub struct Shared {
    pub state: Mutex<State>,
}

lazy_static! {
    pub static ref GLOBAL_NODE_INFO: RwLock<Node> = RwLock::new(Node {
        id: String::from(""),
        host: String::from(""),
        port: 0,
    });
    pub static ref GLOBAL_RAFT_STATE: RwLock<RaftState> = RwLock::new(RaftState {
        role: RaftRole::Follower,
        last_hb: Instant::now()
    });
    pub static ref GLOBAL_RAFT_DATA: RwLock<RaftData> = RwLock::new(RaftData {
        leader_node_id: String::from(""),
        nodes: HashMap::new(),
        buckets: HashMap::new(),
        ts: Instant::now()
    });
}

impl Raft {
    pub fn new() -> Self {
        Raft {
            shared: Arc::new(Shared {
                state: Mutex::new(State {
                    role: RaftRole::Follower,
                    last_hb: Instant::now(),
                }),
            }),
        }
    }

    ///
    /// Check node file exsit or not when server start
    ///
    pub fn init(config: Config) {
        let node_path = format!("{}{}{}", config.datadir, MAIN_SEPARATOR, NODE_FILE_NAME);
        let path = Path::new(&node_path);
        let display = path.display();

        // Existing Node
        if path.exists() {
            let mut file = match File::open(path) {
                Ok(file) => file,
                Err(e) => {
                    panic!("could not open {}: {}", display, e);
                }
            };

            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(s) => {}
                Err(e) => {
                    panic!("could not read file {}: {}", display, e);
                }
            }
            let result = serde_json::from_str::<Node>(&content);
            match result {
                Ok(node) => {
                    println!("Raft::init- {:?}", node);
                    let mut write_lock = GLOBAL_NODE_INFO.write().unwrap();
                    write_lock.host = config.host;
                    write_lock.port = config.port;
                    write_lock.id = node.id;
                    drop(write_lock);
                }
                Err(e) => {
                    panic!("could not parse node file {}:{}", display, e)
                }
            }
        } else {
            // New node
            let node_id = crate::utils::strutil::generate_random_string(16);
            let node = Node {
                id: node_id.clone(),
                host: config.host.clone(),
                port: config.port,
            };
            let result = serde_json::to_string(&node);
            match result {
                Ok(content) => {
                    let mut file = match File::create_new(path) {
                        Ok(file) => file,
                        Err(e) => {
                            panic!("could not create node file: {}:{}", display, e);
                        }
                    };
                    let result = file.write_all(content.as_bytes());
                    match result {
                        Ok(()) => {
                            let mut write_lock = GLOBAL_NODE_INFO.write().unwrap();
                            write_lock.host = config.host;
                            write_lock.port = config.port;
                            write_lock.id = node_id;
                            drop(write_lock);
                        }
                        Err(e) => {
                            panic!("could not write node file: {}:{}", display, e);
                        }
                    }
                }
                Err(e) => {
                    panic!("could not serialize node: {}", e);
                }
            }
        }
    }

    pub async fn start(config: Config) -> Result<(), Box<dyn std::error::Error>> {
        println!("Raft::start");

        let port = config.port + 10000;
        let adrr = format!("{}:{}", config.host, port);
        let tpc_listener = TcpListener::bind(adrr).await?;

        let raft = Raft::new();
        let raft_copy = raft.clone();

        // Accept loop
        let listener = Listener {
            listener: tpc_listener,
            raft: raft.clone(),
        };

        tokio::spawn(async move {
            let _ = crate::raft::vote::Vote::vote(&raft_copy).await;
        });

        tokio::spawn(async move {
            let _ = listener.run().await;
        });

        Ok(())
    }
}

pub struct Listener {
    pub listener: TcpListener,
    pub raft: Raft,
}

impl Listener {
    pub async fn run(self) -> Result<(), std::io::Error> {
        // Accept loop
        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => {
                    println!("raft::server::accept");
                    let mut handler = Handler {
                        socket,
                        raft: self.raft.clone(),
                    };
                    let _ = handler.process().await;
                }
                Err(e) => {
                    println!("raft::Listener::run- {}", e);
                }
            }
        }
    }
}

impl RaftState {
    pub fn role_updated(&self, role: RaftRole) {
        match role {
            RaftRole::Leader => {
                tokio::spawn(async {
                    Heartbeat::send().await;
                });
            }
            RaftRole::Follower => {}
        }
    }
}
