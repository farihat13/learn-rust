use async_prog::create_large_file;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::time::Instant;

#[tokio::main]
async fn main() -> io::Result<()> {
    let input_path = "large_input.txt"; // assume a large file (1GB+)
    let output_path = "large_output.txt";
    if !std::path::Path::new(input_path).exists() {
        create_large_file(input_path, 1024).unwrap();
    }

    let start_time = Instant::now();

    let mut input_file = File::open(input_path).await?;
    let mut output_file = File::create(output_path).await?;

    let mut buffer = vec![0; 1024 * 1024]; // 1MB buffer size

    while let Ok(n) = input_file.read(&mut buffer).await {
        if n == 0 {
            break; // end of file
        }
        output_file.write_all(&buffer[..n]).await?;
    }

    println!(
        "File copied asynchronously in {:?} seconds",
        start_time.elapsed().as_secs_f32()
    );

    Ok(())
}
