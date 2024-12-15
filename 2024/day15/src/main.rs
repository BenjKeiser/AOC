use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::ops::Add;
use std::ops::Mul;
use std::time::Instant;

use std::fmt;
use std::ops::{Deref, DerefMut};

/* Move to Grid Library */
#[derive(Clone, PartialEq, Debug)]
pub struct Grid(Vec<Vec<char>>);

impl Grid {
    /// Creates a new, empty Grid
    pub fn new() -> Self {
        Grid(Vec::new())
    }

    /// Creates a Grid with a predefined size and fills it with a default value
    pub fn with_size(rows: usize, cols: usize, default: char) -> Self {
        let grid = vec![vec![default; cols]; rows];
        Grid(grid)
    }

    pub fn is_move_valid(&self, pos: &Point, dir: &Direction) -> bool {
        match dir {
            Direction { x: -1, y: 0 } => pos.x > 0,
            Direction { x: 0, y: -1 } => pos.y > 0,
            Direction { x: 1, y: 0 } => pos.x < self[0].len() - 1,
            Direction { x: 0, y: 1 } => pos.y < self.len() - 1,
            Direction { x: -1, y: -1 } => pos.x > 0 && pos.y > 0,
            Direction { x: 1, y: -1 } => pos.x < self[0].len() - 1 && pos.y > 0,
            Direction { x: -1, y: 1 } => pos.x > 0 && pos.y < self.len() - 1,
            Direction { x: 1, y: 1 } => pos.x < self[0].len() - 1 && pos.y < self.len() - 1,
            _ => false,
        }
    }
}

impl Deref for Grid {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0 // Deref returns a reference to the inner Vec<Vec<char>>
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for l in self.iter() {
            let row: String = l.iter().collect();
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

pub struct Direction {
    y: i32,
    x: i32,
}

impl Direction {
    fn arrow_char_to_dir(c: &char) -> Option<Direction> {
        match c {
            '<' => Some(Direction { x: -1, y: 0 }),
            '^' => Some(Direction { x: 0, y: -1 }),
            '>' => Some(Direction { x: 1, y: 0 }),
            'v' => Some(Direction { x: 0, y: 1 }),
            _ => None,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Direction {
    type Output = Direction; // Define the result type of the addition

    fn add(self, other: Direction) -> Direction {
        Direction {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Direction {
    type Output = Direction; // Define the result type of the addition

    fn mul(self, scalar: i32) -> Direction {
        Direction {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

/* End of Grid Library */

/* Point Library */
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Add for Point {
    type Output = Point; // Define the result type of the addition

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Option<Point>; // Define the result type of the addition

    fn add(self, other: Direction) -> Option<Point> {
        let x = self.x as i32 + other.x;
        let y = self.y as i32 + other.y;
        if x >= 0 && y >= 0 {
            Some(Point {
                x: x as usize,
                y: y as usize,
            })
        } else {
            None
        }
    }
}

impl Add<&Direction> for Point {
    type Output = Option<Point>; // Define the result type of the addition

    fn add(self, other: &Direction) -> Option<Point> {
        let x = self.x as i32 + other.x;
        let y = self.y as i32 + other.y;
        if x >= 0 && y >= 0 {
            Some(Point {
                x: x as usize,
                y: y as usize,
            })
        } else {
            None
        }
    }
}

impl Mul<usize> for Point {
    type Output = Point; // Define the result type of the addition

    fn mul(self, scalar: usize) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/* End of Point Library */

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Robot {
    pos: Point,
    moves: VecDeque<char>,
}

impl Robot {
    fn move_once(&mut self, grid: &mut Grid) {
        if let Some(m) = self.moves.pop_front() {
            if let Some(d) = Direction::arrow_char_to_dir(&m) {
                if grid.is_move_valid(&self.pos, &d) {
                    if let Some(mut n_pos) = self.pos + &d {
                        if grid[n_pos.y][n_pos.x] == '#' {
                            //we would hit a wall -> no move is possible
                        } else if grid[n_pos.y][n_pos.x] == '.' {
                            //empty space -> we can move
                            grid[self.pos.y][self.pos.x] = '.';
                            self.pos = n_pos;
                            grid[self.pos.y][self.pos.x] = '@';
                        } else if grid[n_pos.y][n_pos.x] == 'O' {
                            let crate_pos = n_pos;
                            while grid.is_move_valid(&n_pos, &d) {
                                if let Some(next) = n_pos + &d {
                                    if grid[next.y][next.x] == '.' {
                                        // move is possible
                                        grid[self.pos.y][self.pos.x] = '.';
                                        self.pos = crate_pos;
                                        grid[self.pos.y][self.pos.x] = '@';
                                        grid[next.y][next.x] = 'O';
                                        break;
                                    } else if grid[next.y][next.x] == 'O' {
                                        // we continue loop
                                        n_pos = next;
                                    } else {
                                        // not possible
                                        break;
                                    }
                                }
                            }
                        } else {
                            //Nothing
                        }
                    }
                }
            }
        }
    }

    fn move_all(&mut self, grid: &mut Grid) {
        while !self.moves.is_empty() {
            self.move_once(grid);
        }
    }
}

fn sum_of_gps(warehouse: &mut Grid, robot: &mut Robot) -> usize {
    let mut sum = 0;
    robot.move_all(warehouse);
    //println!("{warehouse}");

    for (y, row) in warehouse.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'O' {
                sum += y * 100 + x;
            }
        }
    }
    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut moves: VecDeque<char> = VecDeque::new();
    let mut warehouse: Grid = Grid::new();

    let mut y: usize = 0;

    let mut start: Point = Point { y: 0, x: 0 };

    for line in fs::read_to_string(file_path)?.lines() {
        if line.len() > 0 {
            let cs: Vec<_> = line.chars().collect();

            if let Some(x) = cs.iter().position(|&c| c == '@') {
                start = Point { y: y, x: x };
            }

            if cs[0] == '#' {
                warehouse.push(cs);
                y += 1;
            } else {
                moves.append(&mut cs.into());
            }
        }
    }

    let mut robot: Robot = Robot {
        pos: start,
        moves: moves,
    };

    let start = Instant::now();
    let gps = sum_of_gps(&mut warehouse, &mut robot);
    let duration = start.elapsed();
    println!("Part1: {} | {}s", gps, duration.as_secs_f32());

    let start = Instant::now();

    let duration = start.elapsed();
    println!("Part2: {} | {}s", 2, duration.as_secs_f32());

    Ok(())
}
