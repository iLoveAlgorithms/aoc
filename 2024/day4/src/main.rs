use std::fs::File;
use std::io::{self, BufRead};

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),   // Right
    (0, -1),  // Left
    (1, 0),   // Down
    (-1, 0),  // Up
    (1, 1),   // Down-right (diagonal)
    (-1, -1), // Up-left (diagonal)
    (1, -1),  // Down-left (diagonal)
    (-1, 1),  // Up-right (diagonal)
];

const DIRECTIONS_2: [(i32, i32); 4] = [
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);
    let target = "XMAS".chars().collect::<Vec<char>>();  // Target word as a Vec<char>
    let mut grid: Vec<Vec<char>> = Vec::new();

    let mut xmas_num = 0;

    // Part one
    for line in reader.lines() {
        let line = line?; // Handle possible I/O errors
        grid.push(line.chars().collect());
    }

    for (x, row) in grid.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            for &(dx, dy) in DIRECTIONS.iter() {
                if is_target(&grid, x as i32, y as i32, dx, dy, &target) {
                    xmas_num += 1;
                }
            }
        }
    }

    println!("{}", xmas_num);

    // Part two
    let mut x_mas_num = 0;
    let target_2 = "MAS".chars().collect::<Vec<char>>();  // Target word as a Vec<char>

    for (x, row) in grid.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if DIRECTIONS_2.iter().any(|&(d1, d2)| {
                is_target(&grid, x as i32 - d1, y as i32 - d1, d1, d1, &target_2) &&
                is_target(&grid, x as i32 - d2, y as i32 + d2, d2, -d2, &target_2)
            }) {
                x_mas_num += 1;
            }
        }
    }

    println!("{}", x_mas_num);

    Ok(())
}

// Part One
fn is_target(grid: &Vec<Vec<char>>, x: i32, y: i32, dx: i32, dy: i32, t: &Vec<char>) -> bool {
    if (0..t.len()).all(|k| {
        let nx = x + dx * k as i32;
        let ny = y + dy * k as i32;
        in_bounds(grid, nx, ny) &&
        right_letter(grid, nx as usize, ny as usize, t.get(k).expect("Is a char"))
    }) {
        return true;
    }
    false
}

fn in_bounds(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < grid.len() as i32 && y < grid[0].len() as i32
}

fn right_letter(grid: &Vec<Vec<char>>, x: usize, y: usize, c: &char) -> bool {
    match grid.get(x) {
        Some(r) => {
            match r.get(y) {
                Some(&ch) if ch == *c => {
                    return true;
                }
                _ => return false,
            }

        }
        None => return false,
    }
}
