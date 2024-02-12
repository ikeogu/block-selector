use std::fs::File;
use std::io::{self, Read};

pub fn read_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Attempt to decode the bytes using ISO-8859-1 encoding
    let content = String::from_utf8_lossy(&buffer);
    
    Ok(content.into_owned())
}
