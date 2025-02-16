use tokio_stream::{self as stream, StreamExt};

#[tokio::main]
async fn main() {
    let mut numbers = stream::iter(vec![1, 2, 3]);

    // async iteration, e.g, reading from a socket
    while let Some(n) = numbers.next().await {
        println!("Received: {}", n);
    }
}
