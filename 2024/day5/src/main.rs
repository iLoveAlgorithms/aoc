use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);

    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let mut rules_finished = false;

    let mut total_mp = 0;

    for line in reader.lines() {
        let line = line?; // Handle possible I/O errors
        if line.trim().is_empty() {
            rules_finished = true;
            continue;
        }
        
        if !rules_finished {
            let nums: Vec<i32> = line.split("|")
                .map(|s| s.trim().parse::<i32>().expect("Is number"))
                .collect();
            rules.push((*nums.get(0).expect("real"), *nums.get(1).expect("real")));
        } else {
            let nums: Vec<i32> = line.split(",")
                .map(|s| s.trim().parse::<i32>().expect("Is number"))
                .collect();
            updates.push(nums);
        }
    }

    // Part One
    for update in &updates {
        for (i, num) in update.iter().enumerate() {
            if rules.iter().any(|&(x, y)| {
                x == *num
                &&
                match update.iter().position(|&z| z == y) {
                    Some(j) => j < i,
                    None => false,
                }
            }) {
                break;
            }
            if i == update.len() - 1 {
                total_mp += update.get(i / 2).expect("Is real");
            }
        }
    }

    println!("{}", total_mp);

    // Part Two

    let mut new_total_mp = 0;

    let mut bad_updates: Vec<Vec<i32>> = Vec::new();
    for update in &updates {
        for (i, num) in update.iter().enumerate() {
            if rules.iter().any(|&(x, y)| {
                x == *num
                &&
                match update.iter().position(|&z| z == y) {
                    Some(j) => j < i,
                    None => false,
                }
            }) {
                bad_updates.push(update.clone());
                break;
            }
        }
    }

    for update in &bad_updates {
        let sorted_pages = sort(&update, &rules);
        new_total_mp += sorted_pages.get(sorted_pages.len() / 2).expect("Is real");
    }

    println!("{}", new_total_mp);

    Ok(())
}


fn sort(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut new_update = Vec::new();
    for num in update {
        if new_update.len() == 0 {
            new_update.push(*num); 
        } else {
            for i in 0..=new_update.len() {
                if new_update.split_at(i).1.iter().all(|val| {
                    rules.iter().find(|&&(n, v)| n == *num && v == *val).is_some()
                }) {
                    new_update.insert(i, *num);
                    break;
                }
            }
        }
    }
    new_update
}
