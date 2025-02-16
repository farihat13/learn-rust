use async_prog::create_large_file;
use std::fs::File;
use std::io::{self, Read, Write};
use std::time::Instant;

fn main() -> io::Result<()> {
    let input_path = "large_input.txt"; // assume a large file (1GB+)
    let output_path = "large_output.txt";
    if !std::path::Path::new(input_path).exists() {
        create_large_file(input_path, 1024).unwrap();
    }

    let start_time = Instant::now();

    let mut input_file = File::open(input_path)?;
    let mut output_file = File::create(output_path)?;

    let mut buffer = vec![0; 1024 * 1024]; // 1MB buffer

    while let Ok(n) = input_file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        output_file.write_all(&buffer[..n])?;
    }

    println!(
        "File copied synchronously in {:?} seconds",
        start_time.elapsed().as_secs_f32()
    );

    Ok(())
}
