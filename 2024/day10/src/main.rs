use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<u32>> = reader.lines()
        .map(|line|
            line.expect("No errors")
            .chars()
            .map(|char|
                char.to_digit(10).expect("All digits")
            )
            .collect()
        )
        .collect();

    // Part 1
    let _vals: Vec<Vec<u32>> = vec![vec![0; grid[0].len()]; grid.len()];
    let mut trailhead_sum = 0;
    for  (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if grid[i][j] == 0 {
                trailhead_sum += get_trailheads(&grid, 0, i, j).len();
            }
        }
    }
    println!("{}", trailhead_sum);

    // Part 2
    let mut trail_sum = 0;
    for  (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if grid[i][j] == 0 {
                trail_sum += get_trails(&grid, 0, i, j);
            }
        }
    }
    println!("{}", trail_sum);

    Ok(())
}

fn get_trailheads(grid: &Vec<Vec<u32>>, h: u32, i: usize, j: usize) -> HashSet<(usize, usize)> {
    if h == 9 {
        return HashSet::from([(i, j)]);
    }

    let mut all_tops = HashSet::new();

    let mut on_grid_next: Vec<(usize, usize)> = Vec::new();
    if i > 0 { on_grid_next.push((i - 1, j)); }
    if i < grid.len() - 1 { on_grid_next.push((i + 1, j)); }
    if j > 0 { on_grid_next.push((i, j - 1)); }
    if j < grid.len() - 1 { on_grid_next.push((i, j + 1)); }

    for (x, y) in on_grid_next {
        if grid[x][y] == h + 1 {
            all_tops.extend(get_trailheads(grid, h + 1, x, y));
        }
    }
    all_tops
}



fn get_trails(grid: &Vec<Vec<u32>>, h: u32, i: usize, j: usize) -> i32 {
    if h == 9 {
        return 1;
    }

    let mut curr_sum = 0;

    let mut on_grid_next: Vec<(usize, usize)> = Vec::new();
    if i > 0 { on_grid_next.push((i - 1, j)); }
    if i < grid.len() - 1 { on_grid_next.push((i + 1, j)); }
    if j > 0 { on_grid_next.push((i, j - 1)); }
    if j < grid.len() - 1 { on_grid_next.push((i, j + 1)); }

    for (x, y) in on_grid_next {
        if grid[x][y] == h + 1 {
            curr_sum += get_trails(grid, h + 1, x, y)
        }
    }
    curr_sum
}
