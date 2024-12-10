use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<char>> = reader.lines()
        .map(|line|
            line.expect("No errors")
            .chars()
            .collect()
        )
        .collect();
    
    let rowsize = grid.len();
    let colsize = grid[0].len();
    
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let mut antenna_locs: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c != '.' {
                antenna_locs
                    .entry(c)
                    .and_modify(|locs| {
                        locs.push((i, j));
                    })
                    .or_insert(vec![(i, j)]);
            }
        }
    }

    // Part 1
    for (_, locs) in &antenna_locs {
        for a1 in 0..locs.len() {
            for a2 in (a1 + 1)..locs.len() {

                match find_antinode(locs[a1], locs[a2], rowsize, colsize) {
                    Some(coord) => antinodes.insert(coord),
                    None => false,
                };
                match find_antinode(locs[a2], locs[a1], rowsize, colsize) {
                    Some(coord) => antinodes.insert(coord),
                    None => false,
                };
            }
        }
    }

    let antinode_count = antinodes.iter().count();
    println!("{}", antinode_count);

    // Part 2
    for (_, locs) in &antenna_locs {
        for a1 in 0..locs.len() {
            for a2 in (a1 + 1)..locs.len() {
                for coord in get_antinodes(locs[a1], locs[a2], rowsize, colsize) {
                    antinodes.insert(coord);
                }
            }
        }
    }

    let new_antinode_count = antinodes.iter().count();
    println!("{}", new_antinode_count);

    Ok(())
}

fn find_antinode(
    a1: (usize, usize), a2: (usize, usize), rows: usize, cols: usize
) -> Option<(usize, usize)> {
    match (a1.0 * 2).checked_sub(a2.0) {
        Some(r) => {
            if r >= rows {
                return None;
            }
            match (a1.1 * 2).checked_sub(a2.1) {
                Some(c) => {
                    if c >= cols {
                        None
                    } else {
                        Some((r, c))
                    }
                }
                None => None,
            }
        }
        None => None,
    }
}

fn get_antinodes(
    a1: (usize, usize), a2: (usize, usize), rows: usize, cols: usize
) -> Vec<(usize, usize)> {
    let mut antinodes = Vec::new();
    let mut curr_i = a1.0;
    let mut curr_j = a1.1;
    loop {
        antinodes.push((curr_i, curr_j));
        match (curr_i + a1.0).checked_sub(a2.0) {
            Some(r) => {
                if r >= rows {
                    break;
                }
                match (curr_j + a1.1).checked_sub(a2.1) {
                    Some(c) => {
                        if c >= cols {
                            break;
                        } else {
                            curr_i = r;
                            curr_j = c;
                            antinodes.push((r, c));
                        }
                    }
                    None => break,
                }
            }
            None => break,
        }
    }

    curr_i = a2.0;
    curr_j = a2.1;
    loop {
        antinodes.push((curr_i, curr_j));
        match (curr_i + a2.0).checked_sub(a1.0) {
            Some(r) => {
                if r >= rows {
                    break;
                }
                match (curr_j + a2.1).checked_sub(a1.1) {
                    Some(c) => {
                        if c >= cols {
                            break;
                        } else {
                            curr_i = r;
                            curr_j = c;
                            antinodes.push((r, c));
                        }
                    }
                    None => break,
                }
            }
            None => break,
        }
    }
    
    antinodes
}