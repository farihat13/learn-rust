use tonic::{transport::Server, Request, Response, Status};
pub mod calc {
    // tonic::include_proto!("calc");
    include!("proto/calc.rs");
}
use calc::calculator_server::{Calculator, CalculatorServer};
use calc::{EchoMsg, OpReq, OpResp};

#[derive(Default)]
pub struct MyCalculator {}

#[tonic::async_trait]
impl Calculator for MyCalculator {
    async fn echo(
        &self,
        request: tonic::Request<EchoMsg>,
    ) -> std::result::Result<tonic::Response<EchoMsg>, tonic::Status> {
        let req = request.into_inner();
        println!("Request ECHO {}", req.msg);
        Ok(Response::new(EchoMsg {
            msg: req.msg,
        }))
    }
    async fn add(
        &self,
        request: tonic::Request<OpReq>,
    ) -> std::result::Result<tonic::Response<OpResp>, tonic::Status> {
        let req = request.into_inner();
        println!("Request ADD {} {}",  req.num1, req.num2);
        let resp = OpResp {
            result: req.num1 + req.num2,
            error: "".into(),
        };
        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let calculator = MyCalculator::default();

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(CalculatorServer::new(calculator))
        .serve(addr)
        .await?;

    Ok(())
}
