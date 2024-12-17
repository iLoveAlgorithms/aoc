use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

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

    // Part 1
    let mut cache: HashMap<(i64, u32), i64> = HashMap::new();
    let stone_count_2: i64 = starting_arr.iter().map(|stone|
        do_blinks(&mut cache, *stone, 25))
        .sum();
    println!("{}", stone_count_2);

    // Part 2
    let mut cache: HashMap<(i64, u32), i64> = HashMap::new();
    let stone_count_2: i64 = starting_arr.iter().map(|stone|
        do_blinks(&mut cache, *stone, 75))
        .sum();
    println!("{}", stone_count_2);

    Ok(())
}

fn do_blinks(
    cache: &mut HashMap<(i64, u32), i64>,
    stone_val: i64,
    blinks_left: u32,
) -> i64 {
    if blinks_left == 0 {
        return 1;
    }
    if let Some(found) = cache.get(&(stone_val, blinks_left)) {
        return *found;
    }

    let mut result = 0;
    if stone_val == 0 {
        result = do_blinks(cache, 1, blinks_left - 1);
    } else if stone_val.to_string().len() % 2 == 0 {
        let removed_stone = stone_val.to_string();
        let mid = removed_stone.len() / 2;
        let first_half = removed_stone[0..mid].parse::<i64>().expect("nw");
        let second_half = removed_stone[mid..].parse::<i64>().expect("nw");
        result =
            do_blinks(cache, first_half, blinks_left - 1) +
            do_blinks(cache, second_half, blinks_left - 1);
    } else {
        result = do_blinks(cache, stone_val * 2024, blinks_left - 1);
    }

    cache.insert((stone_val, blinks_left), result);
    result
}

/*
// Old helper function
fn next_val(old_num: i64) -> Vec<i64> {
    if old_num == 0 {
        Vec::from([1])
    } else if old_num.to_string().len() % 2 == 0 {
        let removed_stone = old_num.to_string();
        let mid = removed_stone.len() / 2;
        let first_half = removed_stone[0..mid].parse::<i64>().expect("nw");
        let second_half = removed_stone[mid..].parse::<i64>().expect("nw");
        Vec::from([first_half, second_half])
    } else {
        Vec::from([old_num * 2024])
    }
}
*/
/*
// Naive pt 1
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
*/
/*
// Naive way to do it
for blink in 0..=74 {
    let mut i = 0;
    let mut updated: Vec<i64> = Vec::new();

    while i < long_arr.len() {
        let curr_val = long_arr[i];
        if let Some(found) = cache.get(&curr_val) {
            updated.extend(found);
        } else if curr_val == 0 {
            updated.push(1);
        } else if curr_val.to_string().len() % 2 == 0 {
            let removed_stone = curr_val.to_string();
            let mid = removed_stone.len() / 2;
            let first_half = removed_stone[0..mid].parse::<i64>().expect("nw");
            let second_half = removed_stone[mid..].parse::<i64>().expect("nw");
            updated.push(first_half);
            updated.push(second_half);
            add_cache_recurse(&mut cache, curr_val, first_half, second_half);
        } else {
            updated.push(curr_val * 2024);
            cache.insert(curr_val, vec![curr_val * 2024]);
        }

        i += 1;

    }
    println!("{}", blink);

    *long_arr = updated;

}*/