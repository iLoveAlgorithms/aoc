use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_guard(&self, x: &mut usize, y: &mut usize) {
        match self {
            Direction::Up => *x -= 1,
            Direction::Right => *y += 1,
            Direction::Down => *x += 1,
            Direction::Left => *y -= 1,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug, Copy)]
pub struct Guard {
    x: usize,
    y: usize,
    dir: Direction,
}

#[derive(Clone)]
struct GuardPath {
    grid: Vec<Vec<char>>,
    guard: Guard,
    visited: HashSet<Guard>,
}

impl GuardPath {
    fn new(grid: Vec<Vec<char>>, guard: Guard) -> Self {
        GuardPath {
            grid,
            guard,
            visited: HashSet::new(),
        }
    }

    fn exits(&mut self) -> bool {
        loop {
            if !self.visited.insert(self.guard) {
                return false;
            }
            match self.scout() {
                Some('#') => {
                    self.guard.dir = self.guard.dir.turn_right();
                }
                None => break,
                _ => {
                    self.guard.dir.move_guard(&mut self.guard.x, &mut self.guard.y);
                }
            }
        }
        true
    }
    
    fn scout(&self) -> Option<char> {
        if
            self.guard.x == 0 || self.guard.x == self.grid.len() - 1 ||
            self.guard.y == 0 || self.guard.y == self.grid.get(self.guard.x).expect("Real").len() - 1
        {
            return None;
        }
        let mut new_coords = (self.guard.x, self.guard.y);
        self.guard.dir.move_guard(&mut new_coords.0, &mut new_coords.1);
        Some(*self.grid.get(new_coords.0).expect("Real").get(new_coords.1).expect("Real"))
    }

}

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

    let start_x: usize = grid
        .iter()
        .position(|row| row.contains(&'^')).expect("^ in the grid");
    let start_y: usize = grid
        .get(start_x)
        .expect("is a row")
        .iter().position(|v| v == &'^').expect("Real");
    let start_coords = (start_x, start_y);
    let start_dir: Direction = Direction::Up;

    // Part 1
    let finished_grid = move_all(&grid, &start_dir, &start_coords).expect("Part one");

    let x_count = finished_grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&c| c == 'X')
        .count();
    println!("{}", x_count);

    // Part 2
    let mut valid_count = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if *grid.get(i).expect("Real").get(j).expect("Real") == '.' {
                let mut try_grid = grid.clone();
                *try_grid.get_mut(i).expect("Real").get_mut(j).expect("Real") = '#';

                let mut path = GuardPath::new
                    (try_grid, Guard { 
                        x: start_x,
                        y: start_y,
                        dir: start_dir
                     });

                if !path.exits() {
                    valid_count += 1;
                }
            }
        }
    }

    println!("{}", valid_count);
    Ok(())
}

fn move_all(
    grid: &[Vec<char>], start_dir: &Direction, start_coords: &(usize, usize)
) -> Option<Vec<Vec<char>>> {
    let mut new_grid = grid.to_owned();
    let mut dir = *start_dir;
    let mut coords = *start_coords;
    loop {
        *new_grid.get_mut(coords.0).expect("Real").get_mut(coords.1).expect("Real") = 'X';
        match scout(grid, &coords, &dir) {
            Some('#') => {
                dir = dir.turn_right();
            }
            None => break,
            _ => {
                move_guard(&mut coords, &dir);
            }
        }

    }
    Some(new_grid)
}

fn scout(grid: &[Vec<char>], coords: &(usize, usize), dir: &Direction) -> Option<char> {
    if
        coords.0 == 0 || coords.0 == grid.len() - 1 ||
        coords.1 == 0 || coords.1 == grid.get(coords.0).expect("Real").len() - 1
    {
        return None;
    }
    let mut new_coords = (coords.0, coords.1);
    move_guard(&mut new_coords, dir);
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


/*
use std::io::{self, BufRead};
use std::fs::File;
use std::collections::{HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    
    fn move_position(self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }
}

#[derive(Clone)]
struct GuardSimulator {
    grid: Vec<Vec<char>>,
    x: i32,
    y: i32,
    direction: Direction,
    visited: HashSet<(i32, i32, Direction)>, // (x, y, direction)
}

impl GuardSimulator {
    fn new(grid: Vec<Vec<char>>, start_x: i32, start_y: i32, start_direction: Direction) -> Self {
        GuardSimulator {
            grid,
            x: start_x,
            y: start_y,
            direction: start_direction,
            visited: HashSet::new(),
        }
    }

    fn step(&mut self) -> bool {
        let (new_x, new_y) = self.direction.move_position(self.x, self.y);
        if new_x < 0 || new_y < 0 || new_x >= self.grid[0].len() as i32 || new_y >= self.grid.len() as i32 {
            self.x = new_x;
            self.y = new_y;
            return false;
        }

        if self.grid[new_y as usize][new_x as usize] == '#' {
            // Turn right if there's an obstacle
            self.direction = self.direction.turn_right();
        } else {
            // Move forward
            self.x = new_x;
            self.y = new_y;
            if !self.visited.insert((self.x, self.y, self.direction)) {
                // If the guard revisits a position with the same direction, it's a loop.
                return true;
            }
        }

        false
    }

    fn simulate(&mut self) -> bool {
        loop {
            if self.step() {
                return true; // The guard is stuck in a loop
            }
            // Exit condition: if the guard moves out of bounds, we stop
            if self.x < 0 || self.y < 0 || self.x >= self.grid[0].len() as i32 || self.y >= self.grid.len() as i32 {
                return false;
            }
        }
    }
}

fn count_possible_positions_to_add_obstacle(grid: Vec<Vec<char>>, start_x: i32, start_y: i32, start_direction: Direction) -> usize {
    let mut valid_positions = 0;

    // Try placing an obstruction in each empty spot and see if it causes a loop
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '.' && !(x == start_x.try_into().unwrap() && y == start_y.try_into().unwrap()) {
                // Create a new grid with an obstruction at (x, y)
                let mut new_grid = grid.clone();
                new_grid[y][x] = '#';

                let mut simulator = GuardSimulator::new(new_grid, start_x, start_y, start_direction);
                if simulator.simulate() {
                    valid_positions += 1;
                }
            }
        }
    }

    valid_positions
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./input.txt")?;
    let reader = io::BufReader::new(file);

    let grid: Vec<Vec<char>> = reader.lines()
        .map(|line|
            line.expect("No errors")
            .chars()
            .collect()
        )
        .collect();

    let start_x: i32 = grid
        .iter()
        .position(|row| row.contains(&'^')).expect("^ in the grid").try_into().unwrap();
    let start_y: i32 = grid
        .get(start_x as usize)
        .expect("is a row")
        .iter().position(|v| v == &'^').expect("Real").try_into().unwrap();
    let start_direction: Direction = Direction::Up;

    let valid_positions = count_possible_positions_to_add_obstacle(grid, start_x, start_y, start_direction);
    println!("Number of valid positions: {}", valid_positions);
    Ok(())
}

*/