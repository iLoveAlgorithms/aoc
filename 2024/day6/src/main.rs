use std::fs::File;
use std::io::{self, BufRead};

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn main() -> std::io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?; // Handle possible I/O errors

        grid.push(line.chars().collect::<Vec<char>>());
    }

    let guard_x: usize = grid
        .iter()
        .position(|row| row.contains(&'^')).expect("^ in the grid");
    let guard_y: usize = grid
        .get(guard_x)
        .expect("is a row")
        .iter().position(|v| v == &'^').expect("Real");
    let start_coords = (guard_x, guard_y);
    let mut dir: Direction = Direction::Up;
    let mut coords = start_coords;

    // Part 1
    loop {
        *grid.get_mut(coords.0).expect("Real").get_mut(coords.1).expect("Real") = 'X';
        match scout(&grid, &coords, &dir) {
            Some('#') => {
                match dir {
                    Direction::Up => dir = Direction::Right,
                    Direction::Right => dir = Direction::Down,
                    Direction::Down =>  dir = Direction::Left,
                    Direction::Left => dir = Direction::Up,
                }
            }
            None => break,
            _ => {
                move_guard(&mut coords, &dir);
            }
        }

    }

    let x_count = grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == 'X')
        .count();
    println!("{}", x_count);

    Ok(())
}

fn scout(grid: &Vec<Vec<char>>, coords: &(usize, usize), dir: &Direction) -> Option<char> {
    if
        coords.0 == 0 || coords.0 == grid.len() - 1 ||
        coords.1 == 0 || coords.1 == grid.get(coords.0).expect("Real").len() - 1
    {
        return None;
    }
    let mut new_coords = (coords.0, coords.1);
    move_guard(&mut new_coords, &dir);
    Some(*grid.get(new_coords.0).expect("Real").get(new_coords.1).expect("Real"))
}

fn move_guard(coords: &mut (usize, usize), dir: &Direction) {
    match dir {
        Direction::Up => coords.0 -= 1,
        Direction::Right => coords.1 += 1,
        Direction::Down => coords.0 += 1,
        Direction::Left => coords.1 -= 1,
    }
}