use std::fs::File;
use std::io::prelude::*;


pub fn writeToPath(path: &str, data: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}