use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNodeInfo {
    pub node_id: String,
    pub address: String,
    pub role: String, // "LEADER", "FOLLOWER", "CANDIDATE"
    pub status: String, // "ONLINE", "HEALTHY", "SYNCING"
    pub term: u64,
    pub last_heartbeat_ms: u64,
}

pub struct RaftClusterManager {
    node_id: String,
    listen_port: u16,
    peers: RwLock<Vec<String>>,
    nodes_info: RwLock<HashMap<String, ClusterNodeInfo>>,
    current_term: RwLock<u64>,
    is_leader: RwLock<bool>,
}

impl RaftClusterManager {
    pub fn new(node_id: &str, listen_port: u16, peers_csv: &str) -> Self {
        let peers: Vec<String> = peers_csv
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let mut nodes_map = HashMap::new();
        nodes_map.insert(
            node_id.to_string(),
            ClusterNodeInfo {
                node_id: node_id.to_string(),
                address: format!("127.0.0.1:{}", listen_port),
                role: "LEADER".to_string(),
                status: "ONLINE".to_string(),
                term: 1,
                last_heartbeat_ms: 0,
            },
        );

        Self {
            node_id: node_id.to_string(),
            listen_port,
            peers: RwLock::new(peers),
            nodes_info: RwLock::new(nodes_map),
            current_term: RwLock::new(1),
            is_leader: RwLock::new(true),
        }
    }

    pub fn get_node_id(&self) -> &str {
        &self.node_id
    }

    pub fn get_listen_port(&self) -> u16 {
        self.listen_port
    }

    pub fn get_cluster_topology(&self) -> Vec<ClusterNodeInfo> {
        let map = self.nodes_info.read();
        map.values().cloned().collect()
    }

    pub async fn replicate_command(&self, command: &str) -> bool {
        let peers = self.peers.read().clone();
        for peer in peers {
            if peer.contains(&self.listen_port.to_string()) {
                continue;
            }
            let cmd = command.to_string();
            tokio::spawn(async move {
                if let Ok(mut stream) = TcpStream::connect(&peer).await {
                    let msg = format!("{}\n", cmd);
                    let _ = stream.write_all(msg.as_bytes()).await;
                }
            });
        }
        true
    }

    pub async fn start_listener(self: Arc<Self>) {
        let addr = SocketAddr::from(([0, 0, 0, 0], self.listen_port));
        let listener = match TcpListener::bind(addr).await {
            Ok(l) => {
                println!("Consensus / Raft Cluster listener active on {}", addr);
                l
            }
            Err(e) => {
                eprintln!("Warning: Raft listener failed to bind to {}: {}", addr, e);
                return;
            }
        };

        tokio::spawn(async move {
            while let Ok((mut socket, _)) = listener.accept().await {
                tokio::spawn(async move {
                    let mut buf = vec![0u8; 1024];
                    if let Ok(n) = socket.read(&mut buf).await {
                        if n > 0 {
                            let _ = socket.write_all(b"OK\n").await;
                        }
                    }
                });
            }
        });
    }
}

