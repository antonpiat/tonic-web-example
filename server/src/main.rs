use tonic::{Request, Response, Status};
use tonic::transport::Server;
use crate::grpc_web::{EchoRequest, EchoResponse, MathRequest, MathResponse, Operation};
use crate::grpc_web::grpc_web_service_server::{GrpcWebService, GrpcWebServiceServer};

pub mod grpc_web {
    tonic::include_proto!("grpc_web");
}

pub struct GrpcWebServer {}

#[tonic::async_trait]
impl GrpcWebService for GrpcWebServer {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let message = request.into_inner().message;
        let response = EchoResponse {message};

        Ok(Response::new(response))
    }

    async fn do_math(&self, request: Request<MathRequest>) -> Result<Response<MathResponse>, Status> {
        let req = request.into_inner();
        let operation = Operation::try_from(req.operation)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let result = match operation {
            Operation::Add => req.number1 + req.number2,
            Operation::Subtract => req.number1 - req.number2,
            Operation::Multiply => req.number1 * req.number2,
            Operation::Divide => {
                if req.number2 == 0.0 {
                    return Err(Status::invalid_argument("Division by zero"))
                }
                req.number1 / req.number2
            },
        };

        Ok(Response::new(MathResponse { result }))
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let server_url = std::env::var("SERVER_URL")?.parse()?;


    let service = GrpcWebServiceServer::new(GrpcWebServer {});

    Server::builder()
        .accept_http1(true)
        .layer(tower_http::cors::CorsLayer::permissive())
        .layer(tonic_web::GrpcWebLayer::new())
        .add_service(service)
        .serve(server_url)
        .await?;

    Ok(())
}
