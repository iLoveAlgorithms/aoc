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

    println!("{:?}", fs_vals);

    let mut i = fs_vals.len() - 1;
    while j > 0 {
        match fs_vals.get(j) {
            Some(block) => {
                let mut i = 0;
            }
            
        }
        j -= 1;
    }

    // while i < fs_vals.len() {
    //     match fs_vals.get(i) {
    //         Some(block) => {
    //             match block.1 {
    //                 None => {
    //                     if block.0 == 0 {
    //                         i += 1;
    //                         continue;
    //                     }
    //                     let numblocks = block.0;
    //                     while let Some(fill) = fs_vals.get(j) {
    //                         if j <= i { break; }
    //                         println!("{:?} {} {}", fs_vals, i, j);
    //                         match fill.1 {
    //                             Some(_) => {
    //                                 let nummoving = fill.0;
    //                                 // Make it so that splices takes into account the None ahead
    //                                 if nummoving <= numblocks {
    //                                     let remove_block = fs_vals.remove(j);
    //                                     fs_vals.splice(i..=i, vec![remove_block, (numblocks - nummoving, None)]);
    //                                     break;
    //                                 }
    //                             }
    //                             None => (),
    //                         }
    //                         j -= 1;
    //                     }
    //                 }
    //                 Some(_) => ()
    //             }
    //         }
    //         None => (),
    //     }
    //     i += 1;
    // }

    let mut checksum2: i64 = 0;
    let mut c: i64 = 0;
    fs_vals.iter().for_each(|(blocks, val)| {
        if let Some(id) = val {
            checksum2 += (c..c + *blocks as i64).map(|x| x * *id as i64).sum::<i64>();
            c += *blocks as i64;
        }
    });
    println!("{}", checksum2);

    Ok(())
}