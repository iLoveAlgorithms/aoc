use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);

    // Part 1
    let mut tot = 0;
    for line in reader.lines() {
        let line = line?; // Handle possible I/O errors
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        for captures in re.captures_iter(&line) {
            let first: i32 = captures.get(1).expect("Number").as_str().parse().expect("Num");
            let second: i32 = captures.get(2).expect("Number").as_str().parse().expect("Num");
            tot += first * second;
        }

    }

    println!("{}", tot);

    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);
    // Part 2
    let mut cap_on = true;
    let mut tot_2 = 0;
    for line in reader.lines() {
        let line = line?; // Handle possible I/O errors
        let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

        for captures in re.captures_iter(&line) {
            match captures.get(0).expect("real").as_str() {
                "do()" => cap_on = true,
                "don't()" => cap_on = false,
                _ => {
                    if cap_on {
                        let first: i32 = captures.get(1).expect("Number").as_str().parse().expect("Num");
                        let second: i32 = captures.get(2).expect("Number").as_str().parse().expect("Num");
                        tot_2 += first * second;
                    }
                }
            }
        }

    }

    println!("{}", tot_2);


    Ok(())
}

