use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut file = File::create("output.txt").await?;
    file.write_all(b"Hello, world!").await?;

    let mut file = File::open("output.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    println!("File contents: {}", contents);
    Ok(())
}
