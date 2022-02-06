use anyhow::Result;

use tonic::{Request, Response, Status};

use crate::protos::register_server::{Register, RegisterServer};
use crate::protos::{RegisterRequest, RegisterResponse};

#[derive(Clone)]
pub struct RegServer {}

impl RegServer {
    pub async fn new() -> Result<RegisterServer<RegServer>> {
        let server = RegServer {};

        Ok(RegisterServer::new(server))
    }
}

#[tonic::async_trait]
impl Register for RegServer {
    async fn register(
        &self,
        _: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let response = RegisterResponse {
            message: "Hello, World!".to_string(),
        };
        Ok(Response::new(response))
    }
}
