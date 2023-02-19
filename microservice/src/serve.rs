use tonic::{transport::Server, Request, Response, Status};

use api::echo_service_server::{EchoService, EchoServiceServer};
use api::{EchoRequest, EchoResponse};

use ::clap::{Parser};

pub mod api {
    tonic::include_proto!("api");
}

#[derive(Debug, Default)]
pub struct Echo {}

#[tonic::async_trait]
impl EchoService for Echo {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = EchoResponse {
            message: format!("{}", request.into_inner().message),
        };

        Ok(Response::new(reply))
    }
}
                                                                                                            