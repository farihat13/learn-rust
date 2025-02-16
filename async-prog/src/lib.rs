use std::fs::File;
use std::io::{Result, Write};

/// Creates a large file with the given name and size in MB
pub fn create_large_file(file_name: &str, size_mb: usize) -> Result<()> {
    let mut file = File::create(file_name)?;
    let buffer = vec![b'X'; 1024 * 1024]; // 1MB buffer filled with 'X'

    for _ in 0..size_mb {
        file.write_all(&buffer)?; // write 1MB at a time
    }

    file.flush()?; // ensure all data is written to disk
    println!("Large file '{}' ({}MB) created.", file_name, size_mb);

    Ok(())
}
