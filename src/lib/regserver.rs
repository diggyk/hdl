use anyhow::Result;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::commhandler::CommHandler;
use crate::protos::register_server::{Register, RegisterServer};
use crate::protos::{RegisterRequest, RegisterResponse};

/// The Registration Server
///
/// Handles all registration requests that are inbound
#[derive(Clone)]
pub struct RegServer {
    commhandler: Arc<CommHandler>,
}

impl RegServer {
    /// Create a new Registration Server
    pub async fn new(commhandler: Arc<CommHandler>) -> Result<RegisterServer<RegServer>> {
        let server = RegServer { commhandler };

        Ok(RegisterServer::new(server))
    }
}

#[tonic::async_trait]
impl Register for RegServer {
    /// Handle a registration request
    ///
    /// When an incoming request comes in, we can hook in
    /// some kind of validation (TODO) and then add them
    /// to our peers list.
    async fn register(
        &self,
        req: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        println!("Registration Request: {:?}", req);
        println!("Metadata: {:?}", req.remote_addr());

        let ip = match req.remote_addr() {
            Some(ip) => ip,
            None => {
                eprintln!("Client missing remote addr!");
                return Err(Status::invalid_argument("remote addr is invalid"));
            }
        };
        let port = req.into_inner().port;

        let addr = format!("{}:{}", ip.ip(), port);
        self.commhandler
            .add_peer(&addr)
            .await
            .map_err(|err| Status::internal(err.to_string()));

        let response = RegisterResponse {
            accept: true,
            msg: format!("Accepting peer from {}", &addr),
        };

        Ok(Response::new(response))
    }
}
