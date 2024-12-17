use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);

    let starting_arr: Vec<i64> = reader.lines()
        .flat_map(|line| 
            line.expect("No errors")
                .split_whitespace()
                .map(|n|
                    n.parse::<i64>().expect("Valid i64"))
                .collect::<Vec<i64>>()
        )    
        .collect();

    let mut curr_arr = starting_arr.clone();

    for _ in 0..=24 {
        let mut i = 0;
        while i < curr_arr.len() {
            let curr_val = *curr_arr.get(i).expect("exists");
            if curr_val == 0 {
                *curr_arr.get_mut(i).expect("real") = 1;
                i += 1;
            } else if curr_val.to_string().len() % 2 == 0 {
                let removed_stone = curr_arr.remove(i).to_string();
                let mid = removed_stone.len() / 2;
                let first_half = removed_stone[0..mid].parse::<i64>().expect("nw");
                let second_half = removed_stone[mid..].parse::<i64>().expect("nw");
                curr_arr.splice(i..i, vec![first_half, second_half]);
                i += 2;
            } else {
                *curr_arr.get_mut(i).expect("real") *= 2024;
                i += 1;
            }
        }
    }

    let stone_count = curr_arr.len();
    println!("{}", stone_count);

    Ok(())
}