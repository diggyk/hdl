use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::{Channel, Endpoint};
use tonic::Request;

use crate::protos::messages_client::MessagesClient;
use crate::protos::register_client::RegisterClient;
use crate::protos::MessageContent;
use crate::protos::RegisterRequest;

/// Manages connections and messages to remote clients
pub struct CommHandler {
    /// our server port
    our_port: u32,
    /// the peers we are talking to
    peers: Arc<RwLock<Vec<MessagesClient<Channel>>>>,
}

impl CommHandler {
    /// Create a new CommHandler
    pub async fn new(our_port: u32) -> Result<Arc<Self>> {
        let peers = Arc::new(RwLock::new(Vec::new()));
        Ok(Arc::new(Self { our_port, peers }))
    }

    /// Connect to a user
    pub async fn connect(&self, addr_str: &str) -> Result<()> {
        let addr = Endpoint::from_shared(format!("https://{}", addr_str))?;
        let mut client = RegisterClient::connect(addr.clone()).await?;
        let request = Request::new(RegisterRequest {
            port: self.our_port,
        });

        let response = client.register(request).await?;

        if response.into_inner().accept {
            self.add_peer(addr_str).await?;
        }

        Ok(())
    }

    /// Add a client to a peer based on addr
    pub async fn add_peer(&self, addr_str: &str) -> Result<()> {
        let addr = Endpoint::from_shared(format!("https://{}", addr_str))?;
        let client = MessagesClient::connect(addr.clone()).await?;
        self.peers.write().await.push(client);
        Ok(())
    }

    /// Send a message to all our peers
    pub async fn send_message(&self, msg: &str) -> Result<()> {
        for peer in self.peers.write().await.iter_mut() {
            let request = Request::new(MessageContent {
                msg: msg.to_string(),
            });
            peer.msg(request).await?;
        }

        Ok(())
    }
}
