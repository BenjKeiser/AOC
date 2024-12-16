use grid::{Direction, Grid, Point};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::time::Instant;
use std::usize::MAX;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Maze {
    start: (Point, Direction),
    end: Point,
    maze: Grid,
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.maze)?;
        writeln!(f, "{} -> {}", self.start.0, self.start.1)?;
        writeln!(f, "{}", self.end,)?;
        Ok(())
    }
}

impl Maze {
    fn make_move(
        self: &Self,
        pos: &Point,
        dir: &Direction,
        path: &Vec<Point>,
        cost: usize,
        cost_map: &mut HashMap<(Point, Direction), usize>,
    ) -> Option<(Vec<Point>, usize)> {
        let mut new_path: Vec<Point> = path.clone();
        if self.maze.is_move_valid(pos, dir) {
            let next: Point = (*pos + dir).unwrap();
            let c = cost + 1;
            if self.maze[next.y][next.x] == '#' {
                //we cannot move here
                return None;
            }

            //update cost matrix
            if let Some(current_value) = cost_map.get_mut(&(next, *dir)) {
                if *current_value > c {
                    *current_value = c;
                } else {
                    //we were already here but cheaper -> we can stop
                    return None;
                }
            } else {
                cost_map.insert((next, *dir), c);
            }

            //check if end is reached
            new_path.push(next);
            if next == self.end {
                return Some((new_path, c));
            }

            let mut cost_min = MAX;
            let mut ret_path: Vec<Point> = Vec::new();
            //make further moves
            if let Some((p, p_c)) = self.make_move(&next, &dir, &new_path, c, cost_map) {
                if p_c < cost_min {
                    ret_path = p;
                    cost_min = p_c;
                }
            }

            if let Some((p, p_c)) = self.make_move(
                &next,
                &(dir.turn_left().unwrap()),
                &new_path,
                c + 1000,
                cost_map,
            ) {
                if p_c < cost_min {
                    ret_path = p;
                    cost_min = p_c;
                }
            }

            if let Some((p, p_c)) = self.make_move(
                &next,
                &(dir.turn_right().unwrap()),
                &new_path,
                c + 1000,
                cost_map,
            ) {
                if p_c < cost_min {
                    ret_path = p;
                    cost_min = p_c;
                }
            }
            if !ret_path.is_empty() {
                return Some((ret_path, cost_min));
            }
        }

        None
    }

    fn solve(self: &Self) -> (Vec<Point>, usize) {
        let mut path: Vec<Point> = Vec::new();
        let mut cost: usize = MAX;

        let mut cost_map: HashMap<(Point, Direction), usize> = HashMap::new();

        cost_map.insert(self.start, 0);
        if let Some((p, c)) =
            self.make_move(&self.start.0, &self.start.1, &Vec::new(), 0, &mut cost_map)
        {
            if c < cost {
                cost = c;
                path = p;
            }
        }
        
        let mut turn = self.start.1.turn_left().unwrap();
        cost_map.insert((self.start.0, turn), 1000);
        if let Some((p, c)) = self.make_move(
            &self.start.0,
            &turn,
            &Vec::new(),
            1000,
            &mut cost_map,
        ) {
            if c < cost {
                cost = c;
                path = p;
            }
        }
        turn = turn.turn_left().unwrap();
        cost_map.insert((self.start.0, turn), 2000);
        if let Some((p, c)) = self.make_move(
            &self.start.0,
            &turn,
            &Vec::new(),
            2000,
            &mut cost_map,
        ) {
            if c < cost {
                cost = c;
                path = p;
            }
        }

        turn = turn.turn_right().unwrap();
        cost_map.insert((self.start.0, turn), 1000);
        if let Some((p, c)) = self.make_move(
            &self.start.0,
            &turn,
            &Vec::new(),
            1000,
            &mut cost_map,
        ) {
            if c < cost {
                cost = c;
                path = p;
            }
        }

        (path, cost)
    }
}

fn solve_maze(maze: &mut Maze) -> usize {
    maze.solve().1
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut maze: Grid = Grid::new();

    let mut y: usize = 0;

    let mut start: Point = Point { y: 0, x: 0 };
    let mut end: Point = Point { y: 0, x: 0 };

    for line in fs::read_to_string(file_path)?.lines() {
        if line.len() > 0 {
            let cs: Vec<_> = line.chars().collect();

            if let Some(x) = cs.iter().position(|&c| c == 'S') {
                start = Point { y: y, x: x };
            }

            if let Some(x) = cs.iter().position(|&c| c == 'E') {
                end = Point { y: y, x: x };
            }

            maze.push(cs);
        }
        y += 1;
    }

    let mut maze: Maze = Maze {
        start: (start, Direction { x: 1, y: 0 }),
        end: end,
        maze: maze,
    };

    println!("{}", maze);

    let start = Instant::now();
    let score = solve_maze(&mut maze);
    let duration = start.elapsed();
    println!("Part1: {} | {}s", score, duration.as_secs_f32());

    let start = Instant::now();

    let duration = start.elapsed();
    println!("Part2: {} | {}s", 2, duration.as_secs_f32());

    Ok(())
}
