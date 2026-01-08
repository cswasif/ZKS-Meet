// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use libp2p::{
    core::upgrade,
    futures::StreamExt,
    mdns,
    noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, Swarm,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PMessage {
    pub content: String,
    pub sender: String,
    pub timestamp: String,
}

#[derive(NetworkBehaviour)]
struct P2PNetworkBehaviour {
    mdns: mdns::tokio::Behaviour,
}

struct P2PNetwork {
    swarm: Arc<Mutex<Swarm<P2PNetworkBehaviour>>>,
    peers: Arc<Mutex<HashSet<PeerId>>>,
}

impl P2PNetwork {
    fn new() -> Self {
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        info!("Local peer id: {:?}", local_peer_id);

        let transport = tcp::tokio::Transport::default()
            .upgrade(upgrade::Version::V1Lazy)
            .authenticate(noise::Config::new(&local_key).expect("signing libp2p-noise static keypair"))
            .multiplex(yamux::Config::default())
            .boxed();

        let behaviour = P2PNetworkBehaviour {
            mdns: mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id).expect("Failed to create mDNS"),
        };

        let swarm = Swarm::with_tokio_executor(transport, behaviour, local_peer_id);
        
        Self {
            swarm: Arc::new(Mutex::new(swarm)),
            peers: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    async fn start(&mut self) -> Result<String, String> {
        info!("Starting P2P network...");
        
        // Listen on all interfaces
        let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/0".parse().map_err(|e| format!("Parse error: {}", e))?;
        
        self.swarm.lock().await.listen_on(listen_addr).map_err(|e| format!("Listen error: {}", e))?;
        
        info!("P2P network started successfully");
        Ok("P2P network started".to_string())
    }

    async fn discover_peers(&self) -> Result<Vec<String>, String> {
        let peers = self.peers.lock().await;
        let peer_ids: Vec<String> = peers.iter().map(|p| p.to_string()).collect();
        info!("Discovered {} peers", peer_ids.len());
        Ok(peer_ids)
    }

    async fn send_message(&self, message: P2PMessage) -> Result<String, String> {
        info!("Sending message: {:?}", message);
        // In a real implementation, this would broadcast to all peers
        Ok(format!("Message sent to {} peers", self.peers.lock().await.len()))
    }
}

#[tauri::command]
async fn test_connection() -> Result<String, String> {
    info!("Testing connection to Tauri backend");
    Ok("Tauri backend connection successful".to_string())
}

#[tauri::command]
async fn connect_p2p(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let mut network = state.p2p_network.lock().await;
    match network.start().await {
        Ok(_) => Ok("P2P network connection established successfully".to_string()),
        Err(e) => Err(format!("Failed to connect to P2P network: {}", e))
    }
}

#[tauri::command]
async fn discover_peers(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let network = state.p2p_network.lock().await;
    network.discover_peers().await
}

#[tauri::command]
async fn send_message(state: tauri::State<'_, AppState>, content: String) -> Result<String, String> {
    let network = state.p2p_network.lock().await;
    let message = P2PMessage {
        content,
        sender: "local".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };
    network.send_message(message).await
}

struct AppState {
    p2p_network: Arc<Mutex<P2PNetwork>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info,libp2p=debug")
        .init();

    info!("Starting Edge Hive - ZKS Protocol P2P Application");

    let p2p_network = Arc::new(Mutex::new(P2PNetwork::new()));
    
    // Start the P2P network event loop in the background
    let p2p_clone = Arc::clone(&p2p_network);
    tokio::spawn(async move {
        run_p2p_event_loop(p2p_clone).await;
    });

    let app_state = AppState {
        p2p_network: Arc::clone(&p2p_network),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            test_connection,
            connect_p2p,
            discover_peers,
            send_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

async fn run_p2p_event_loop(network: Arc<Mutex<P2PNetwork>>) {
    info!("Starting P2P event loop");
    
    loop {
        // In a real implementation, this would handle network events
        // For now, we'll just keep the network alive and periodically discover peers
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
        
        // Simulate periodic peer discovery
        if let Ok(peers) = network.lock().await.discover_peers().await {
            info!("Periodic discovery found {} peers", peers.len());
        }
    }
}