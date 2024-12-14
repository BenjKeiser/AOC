use regex::Regex;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::ops::Add;
use std::ops::Mul;
use std::time::Instant;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Robot {
    p: Point,
    v: Point,
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

impl Mul<i64> for Point {
    type Output = Point; // Define the result type of the addition

    fn mul(self, scalar: i64) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Point {
    fn normalize(&mut self, width: usize, height: usize) {
        self.x = self.x % width as i64;
        self.y = self.y % height as i64;
        if self.x < 0 {
            self.x += width as i64;
        }
        if self.y < 0 {
            self.y += height as i64;
        }
    }

    fn get_quadrant(self, width: usize, height: usize) -> Option<u64> {
        if self.x >= 0 && self.x < width as i64 / 2 {
            if self.y >= 0 && self.y < height as i64 / 2 {
                return Some(1);
            } else if self.y >= height as i64 / 2 + 1 && self.y < height as i64 {
                return Some(2);
            } else {
                return None;
            }
        } else if self.x >= width as i64 / 2 + 1 && self.x < width as i64 {
            if self.y >= 0 && self.y < height as i64 / 2 {
                return Some(3);
            } else if self.y >= height as i64 / 2 + 1 && self.y < height as i64 {
                return Some(4);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }
}

impl Robot {
    fn move_robot(self, times: i64) -> Point {
        self.p + self.v * times
    }

    fn move_robot_mut(&mut self, times: i64) {
        self.p = self.p + self.v * times
    }
}

fn print_robots(robots: &Vec<Point>, width: usize, height: usize) {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width]; height];
    for r in robots {
        grid[r.y as usize][r.x as usize] = '#';
    }
    for l in grid {
        println!("{}", l.iter().cloned().collect::<String>());
    }
}

fn get_safety_factor(robots: &Vec<Robot>, moves: i64, width: usize, height: usize) -> usize {
    let mut grid_pos: Vec<Point> = robots.iter().map(|r| r.move_robot(moves)).collect(); //get grid positions
    grid_pos.iter_mut().for_each(|p| p.normalize(width, height));

    let quadrants: Vec<_> = grid_pos
        .iter()
        .filter_map(|p| p.get_quadrant(width, height))
        .collect();

    quadrants.iter().filter(|&&p| p == 1).count()
        * quadrants.iter().filter(|&&p| p == 2).count()
        * quadrants.iter().filter(|&&p| p == 3).count()
        * quadrants.iter().filter(|&&p| p == 4).count()
}

fn check_neighbours(robots: &Vec<Point>) -> bool {
    for r in robots {
        if robots.contains(&(*r + Point { x: -1, y: 1 }))
            && robots.contains(&(*r + Point { x: 0, y: 1 }))
            && robots.contains(&(*r + Point { x: 1, y: 1 }))
            && robots.contains(&(*r + Point { x: -1, y: 0 }))
            && robots.contains(&(*r + Point { x: 1, y: 0 }))
            && robots.contains(&(*r + Point { x: -1, y: -1 }))
            && robots.contains(&(*r + Point { x: 0, y: -1 }))
            && robots.contains(&(*r + Point { x: 1, y: -1 }))
        {
            return true;
        }
    }
    false
}

fn find_tree(robots: &Vec<Robot>, moves: i64, width: usize, height: usize) -> i64 {
    let mut rs = robots.clone();

    let mut i = 0;
    while i < moves {
        rs.iter_mut().for_each(|r| r.move_robot_mut(1));
        let mut grid_pos: Vec<Point> = rs.iter().map(|r| r.p).collect(); //get grid positions
        grid_pos.iter_mut().for_each(|p| p.normalize(width, height));
        i += 1;
        if check_neighbours(&grid_pos) {
            print_robots(&grid_pos, width, height);
            return i;
        }
    }
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let input = fs::read_to_string(file_path)?;

    // Regex to capture integers after X or Y markers
    let re = Regex::new(r"p=([+-]?\d+),([+-]?\d+) v=([+-]?\d+),([+-]?\d+)").unwrap();

    // Collect all groups
    let robots: Vec<Robot> = re
        .captures_iter(&input) // Iterate over all matches
        .filter_map(|cap| {
            Some(Robot {
                p: Point {
                    x: cap[1].parse::<i64>().ok()?, // p.x
                    y: cap[2].parse::<i64>().ok()?, // p.y
                },
                v: Point {
                    x: cap[3].parse::<i64>().ok()?, // v.x
                    y: cap[4].parse::<i64>().ok()?, // v.y
                },
            })
        })
        .collect();

    //println!("{:?}", robots);

    let start = Instant::now();
    //let sf = get_safety_factor(&robots, 100, 11, 7); //-> test_input.txt
    let sf = get_safety_factor(&robots, 100, 101, 103);
    let duration = start.elapsed();
    println!("Part1: {} | {}s", sf, duration.as_secs_f32());

    let start = Instant::now();
    let iters = find_tree(&robots, 10000, 101, 103);
    let duration = start.elapsed();
    println!("Part2: {} | {}s", iters, duration.as_secs_f32());

    Ok(())
}
