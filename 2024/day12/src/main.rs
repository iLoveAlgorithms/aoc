use std::fs::File;
use std::io::{self, BufRead};
// use std::collections::HashMap;
use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 4] = [
    (0, 1),   // Right
    (0, -1),  // Left
    (1, 0),   // Down
    (-1, 0),  // Up
];

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

    let mut total_cost = 0;
    let mut regions: Vec<HashSet<(u32, u32)>> = Vec::new();
    let mut all_cells: HashSet<(u32, u32)> = HashSet::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if !all_cells.insert((i.try_into().unwrap(), j.try_into().unwrap())) {
                continue;
            }

            let mut new_region: HashSet<(u32, u32)> = HashSet::new();
            let mut queue: Vec<(u32, u32)> = Vec::from(
                [(i.try_into().unwrap(), j.try_into().unwrap())]
            );
            while let Some(v) = queue.pop() {
                if !new_region.insert(v) {
                    continue;
                }

                new_region.insert(v);
                queue.extend(get_neighbours(&grid, v.0, v.1));
            }

            all_cells.extend(new_region.clone());
            regions.push(new_region);
        }
    }

    for region in regions {
        let mut perimeter = region
            .iter()
            .fold(0, |acc, &(x, y)| acc + 4 - get_neighbours(&grid, x, y).len());
        total_cost += region.len() * perimeter;
    }

    println!("{}", total_cost);

    Ok(())
}

fn get_neighbours(grid: &Vec<Vec<char>>, x: u32, y: u32) -> Vec<(u32, u32)> {
    DIRECTIONS
        .iter()
        .filter_map(move |&(dx, dy)| {
            let nx: i32 = x as i32 + dx;
            let ny: i32 = y as i32 + dy;
            if in_region(&grid, x, y, nx, ny) {
                Some((nx as u32, ny as u32))
            } else {
                None
            }
        })
        .collect()
}

fn in_region(grid: &Vec<Vec<char>>, x: u32, y: u32, nx: i32, ny: i32) -> bool {
    if {
        in_bounds(grid, nx, ny) &&
        grid[nx as usize][ny as usize] == grid[x as usize][y as usize]
    } {
        return true;
    }
    false
}


fn in_bounds(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < grid.len() as i32 && y < grid[0].len() as i32
}
