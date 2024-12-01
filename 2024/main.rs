use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("../input.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?; // Handle possible I/O errors
    }

    Ok(())
}