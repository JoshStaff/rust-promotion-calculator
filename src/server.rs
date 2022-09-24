use tonic::{transport::Server, Request, Response, Status};

use calculator::calculator_server::{Calculator, CalculatorServer};
use calculator::{CalculationData, CalculationResponse};

pub mod calculator {
    tonic::include_proto!("calculator");
}

#[derive(Debug, Default)]
pub struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn run(
        &self,
        request: Request<CalculationData>,
    ) -> Result<Response<CalculationResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = CalculationResponse {
            price: 1
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let calculator_service = CalculatorService::default();

    Server::builder()
        .add_service(CalculatorServer::new(calculator_service))
        .serve(addr)
        .await?;

    Ok(())
}