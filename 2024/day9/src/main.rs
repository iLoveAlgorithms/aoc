use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);

    let mut fs: Vec<Option<i32>> = Vec::new();
    let mut is_file = true;
    let mut id = 0;

    for line in reader.lines() {
        let line = line?; // Handle possible I/O errors
        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                if is_file {
                    fs.extend(std::iter::repeat(Some(id)).take(digit as usize));
                    id += 1;
                } else {
                    fs.extend(std::iter::repeat(None).take(digit as usize));
                }
                is_file = !is_file;
            }
        }
    }

    let mut i = 0;
    while i < fs.len() {
        match fs.get(i) {
            Some(space) => {
                match space {
                    None => {
                        while let Some(fill) = fs.pop() {
                            match fill {
                                Some(value) => {
                                    *fs.get_mut(i).expect("Real") = Some(value);
                                    break;
                                }
                                None => (),
                            }
                        }
                    }
                    Some(_) => ()
                }
            }
            None => break,
        }


        i += 1;
    }

    let checksum = fs.iter().enumerate()
        .fold(0, |acc, (i, id)| acc + i * id.expect("is i32") as usize);
    println!("{}", checksum);

    Ok(())
}