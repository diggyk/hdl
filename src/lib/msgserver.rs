use anyhow::Result;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::commhandler::CommHandler;
use crate::protos::messages_server::{Messages, MessagesServer};
use crate::protos::{MessageContent, MessageResponse};

/// The Message Server
///
/// Handles all inbound messages
#[derive(Clone)]
pub struct MsgServer {
    // commhandler: Arc<CommHandler>,
}

impl MsgServer {
    /// Create a new Registration Server
    pub async fn new(
        // commhandler: Arc<CommHandler>
    ) -> Result<MessagesServer<MsgServer>> {
        let server = MsgServer { 
            // commhandler 
        };

        Ok(MessagesServer::new(server))
    }
}

#[tonic::async_trait]
impl Messages for MsgServer {
    /// Handle an incoming message
    async fn msg(
        &self,
        req: Request<MessageContent>,
    ) -> Result<Response<MessageResponse>, Status> {
        println!("{}", req.into_inner().msg);

        let response = MessageResponse {
            msg: "Gotcha".to_string(),
        };

        Ok(Response::new(response))
    }
}
