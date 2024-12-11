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

    // Part 1
    let mut i = 0;
    while i < fs.len() {
        match fs.get(i) {
            Some(block) => {
                match block {
                    None => {
                        while let Some(fill) = fs.pop() {
                            if i >= fs.len() { break; }
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

    // Part 2
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);
    let mut fs_vals: Vec<(u32, Option<i32>)> = reader.lines()
        .flat_map(|line| {
            match line {
                Ok(line) => Some(
                    line.chars()
                        .filter_map(|c| c.to_digit(10))
                        .enumerate()
                        .map(|(i, digit)| {
                            if i % 2 == 0 {
                                (digit, Some(i as i32 / 2))
                            } else {
                                (digit, None)
                            }
                        })
                        .collect::<Vec<_>>()
                ),
                Err(_) => None,
            }
        })
        .flat_map(|line_vals| line_vals.into_iter())
        .collect();

    let mut i = fs_vals.len() - 1;
    while i > 0 {
        if let Some(file) = fs_vals.get(i) {
            if let Some(_id) = file.1 {
                let mut j = 0;
                while j < i {
                    if let Some(empty) = fs_vals.get(j) {
                        if empty.1.is_none() && empty.0 >= file.0 {
                            let spsize = empty.0;
                            let rmsize = file.0;
                            let remove_block = fs_vals.remove(i);
                            fs_vals.splice(j..=j, vec![remove_block, (spsize - rmsize, None)]);
                            fs_vals.insert(i, (rmsize, None));
                            break;
                        }
                    }
                    j += 1;
                }
            }

        }
        i -= 1;
    }

    let mut checksum2: i64 = 0;
    let mut c: i64 = 0;

    for (blocks, val) in fs_vals {
        let blocks_i64 = blocks as i64;
        match val {
            Some(id) => {
                let sum: i64 = (c..c + blocks_i64).map(|x| x * id as i64).sum();
                checksum2 += sum;
            }
            None => (),
        }
        c += blocks_i64;
    }

    println!("{}", checksum2);

    Ok(())
}