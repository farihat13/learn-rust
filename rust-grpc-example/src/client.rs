pub mod calc {
    // tonic::include_proto!("calc");
    include!("proto/calc.rs");
}
use calc::calculator_client::CalculatorClient;
use calc::{EchoMsg, OpReq};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CalculatorClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(EchoMsg {
        msg: "Hello!".into(),
    });
    let response = client.echo(request).await?;
    println!("Echo: {:?}", response.into_inner().msg);

    let request = tonic::Request::new(OpReq { num1: 3, num2: 4 });
    let response = client.add(request).await?;
    let resp = response.into_inner();
    println!("Add: {} {}", resp.result, resp.error);

    Ok(())
}
