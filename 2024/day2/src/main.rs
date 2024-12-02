use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);

    let levels: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| {
            line
            .expect("should be real")
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Integer inputs"))
            .collect::<Vec<i32>>()
        })
        .collect();

    let num_safe = levels.iter().filter(|lvl| is_safe(lvl)).count();

    println!("{}", num_safe);

    let damp_safe = levels.iter().filter(|lvl| dampened(lvl)).count();

    println!("{}", damp_safe);

    Ok(())
}

// Part 1
fn is_safe(level: &[i32]) -> bool {
    let mut asc = true;
    let mut desc = true;

    for window in level.windows(2) {
        if let [a, b] = window {
            match (a - b).abs() {
                ..=0 => return false,
                4.. => return false,
                _ => (),
            }

            if a > b {
                asc = false;
            }
            if a < b {
                desc = false;
            }
        }
    }

    asc || desc
}

// Part 2
fn dampened(level: &[i32]) -> bool {
    if is_safe(level) {
        return true;
    }

    for i in 0..level.len() {
        let mut new_level = Vec::with_capacity(level.len() - 1);
        new_level.extend_from_slice(&level[0..i]);
        new_level.extend_from_slice(&level[i + 1..]);

        if is_safe(&new_level) {
            return true;
        }
    }

    false
}

/*
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);

    let mut num_safe = 0;

    for line in reader.lines() {
        let line = line?; // Handle possible I/O errors
        let level: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Integer inputs"))
            .collect();

        let mut safe = true;
        let mut asc = true;

        if let Some(&[a,b]) = level.windows(2).next() {
            match a - b {
                ..=-1 => asc = false,
                0 => safe = false,
                _ => (),
            }
        }

        for window in level.windows(2) {
            if let [a, b] = window {
                match a - b {
                    ..=-1 if asc => safe = false,
                    4.. if asc => fase = false,
                    1.. if !asc => safe = false,
                    ..=-4 if !asc => safe = false,
                    0 => safe = false,
                    _ => (),
                }
            }
        }

        if safe { num_safe += 1; }
    }

    println!("{}", num_safe);

    Ok(())
}
*/