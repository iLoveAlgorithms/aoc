use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);

    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?; // Handle possible I/O errors
        let words: Vec<&str> = line.split_whitespace().collect(); // Split the line by whitespace

        vec1.push(words.get(0).expect("Each line should have 2 words").parse().unwrap());
        vec2.push(words.get(1).expect("Each line should have 2 words").parse().unwrap());
    }

    // Part 1

    vec1.sort();
    vec2.sort();

    let mut total_dists: i32 = 0;
    for (v1, v2) in vec1.iter().zip(vec2.iter()) {
      total_dists += (v1 - v2).abs();
    }

    println!("{}", total_dists);

    // Part 2

    let mut frequencies = HashMap::new();
    for item in vec2 {
        frequencies.entry(item)
            .and_modify(|v| {
                *v += 1
            })
            .or_insert(1);
    }

    let mut total_similarity = 0;
    for v1 in vec1.iter() {
        total_similarity += frequencies.get(v1).unwrap_or(&0) * v1;  // Default to 0 if not found

    }

    println!("{}", total_similarity);

    Ok(())
}

