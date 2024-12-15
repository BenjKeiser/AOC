use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::time::Instant;
use grid::{Grid, Point, Direction};

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
                        } else if grid[n_pos.y][n_pos.x] == '[' || grid[n_pos.y][n_pos.x] == ']' {
                            self.move_double_crates(grid, self.pos, d);
                        } else {
                            //Nothing
                        }
                    }
                }
            }
        }
    }

    fn move_double_crates_up_or_down(
        &self,
        grid: &mut Grid,
        crates: &Vec<(Point, Point)>,
        dir: &Direction,
    ) -> bool {
        let mut next_crates: Vec<(Point, Point)> = Vec::new();

        for c in crates {
            if grid.is_move_valid(&c.0, dir) {
                if let Some(nl) = c.0 + dir {
                    if let Some(nr) = c.1 + dir {
                        if grid[nl.y][nl.x] == '#' || grid[nr.y][nr.x] == '#' {
                            //no move possible
                            return false;
                        } else if grid[nl.y][nl.x] == '[' || grid[nr.y][nr.x] == ']' {
                            next_crates.push((nl, nr));
                        } else if grid[nl.y][nl.x] == ']' {
                            let cr: (Point, Point) = (
                                Point {
                                    x: nl.x - 1,
                                    y: nl.y,
                                },
                                nl,
                            );
                            if !next_crates.contains(&cr) {
                                next_crates.push(cr);
                            }
                            if grid[nr.y][nr.x] == '[' {
                                let cr: (Point, Point) = (
                                    nr,
                                    Point {
                                        x: nr.x + 1,
                                        y: nr.y,
                                    },
                                );
                                if !next_crates.contains(&cr) {
                                    next_crates.push(cr);
                                }
                            }
                        } else if grid[nr.y][nr.x] == '[' {
                            let cr: (Point, Point) = (
                                nr,
                                Point {
                                    x: nr.x + 1,
                                    y: nr.y,
                                },
                            );
                            if !next_crates.contains(&cr) {
                                next_crates.push(cr);
                            }
                        }
                    }
                }
            } else {
                return false;
            }
        }

        if !next_crates.is_empty() {
            if !self.move_double_crates_up_or_down(grid, &next_crates, dir) {
                return false;
            }
        }
        

        //we get here without an early abort -> we can move the crates
        for c in crates {
            if let Some(nl) = c.0 + dir {
                if let Some(nr) = c.1 + dir {
                    grid[nl.y][nl.x] = grid[c.0.y][c.0.x];
                    grid[nr.y][nr.x] = grid[c.1.y][c.1.x];
                    grid[c.0.y][c.0.x] = '.';
                    grid[c.1.y][c.1.x] = '.';
                }
            }
        }

        true
    }

    fn move_double_crates(&mut self, grid: &mut Grid, r_pos: Point, dir: Direction) {
        if dir.y != 0 {
            if grid.is_move_valid(&r_pos, &dir) {
                if let Some(n) = r_pos + &dir {
                    if grid[n.y][n.x] == '[' {
                        if self.move_double_crates_up_or_down(
                            grid,
                            &vec![(n, Point { x: n.x + 1, y: n.y })],
                            &dir,
                        ) {
                            grid[n.y][n.x] = grid[r_pos.y][r_pos.x];
                            grid[r_pos.y][r_pos.x] = '.';
                            self.pos = n;
                        }
                    } else if grid[n.y][n.x] == ']' {
                        if self.move_double_crates_up_or_down(
                            grid,
                            &vec![(Point { x: n.x - 1, y: n.y }, n)],
                            &dir,
                        ) {
                            grid[n.y][n.x] = grid[r_pos.y][r_pos.x];
                            grid[r_pos.y][r_pos.x] = '.';
                            self.pos = n;
                        }
                    } else {
                    }
                }
            }
            // up or down -> it's tricky
        } else {
            let mut n_pos = r_pos;
            while grid.is_move_valid(&n_pos, &dir) {
                if let Some(next) = n_pos + &dir {
                    if grid[next.y][next.x] == '.' {
                        // move is possible
                        n_pos = next.clone();
                        while n_pos != r_pos {
                            if let Some(n) = n_pos - &dir {
                                grid[n_pos.y][n_pos.x] = grid[n.y][n.x];
                                n_pos = n;
                            } else {
                                break;
                            }
                        }
                        grid[r_pos.y][r_pos.x] = '.';
                        if let Some(n) = r_pos + &dir {
                            self.pos = n;
                        }
                        break;
                    } else if grid[next.y][next.x] == '[' || grid[next.y][next.x] == ']' {
                        // we continue loop
                        n_pos = next;
                    } else {
                        // not possible
                        break;
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

    for (y, row) in warehouse.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'O' {
                sum += y * 100 + x;
            }
        }
    }

    for (y, row) in warehouse.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '[' {
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
    let mut warehouse_p2: Grid = Grid::new();

    let mut y: usize = 0;

    let mut start: Point = Point { y: 0, x: 0 };

    for line in fs::read_to_string(file_path)?.lines() {
        if line.len() > 0 {
            let cs: Vec<_> = line.chars().collect();
            let mut cs2: Vec<char> = Vec::new();

            if let Some(x) = cs.iter().position(|&c| c == '@') {
                start = Point { y: y, x: x };
            }

            if cs[0] == '#' {
                warehouse.push(cs);
                for c in line.chars() {
                    if c == 'O' {
                        cs2.push('[');
                        cs2.push(']');
                    } else if c == '@' {
                        cs2.push('@');
                        cs2.push('.');
                    } else {
                        cs2.push(c);
                        cs2.push(c);
                    }
                }
                warehouse_p2.push(cs2);
                y += 1;
            } else {
                moves.append(&mut cs.into());
            }
        }
    }

    let mut robot: Robot = Robot {
        pos: start,
        moves: moves.clone(),
    };

    let mut robot_p2: Robot = Robot {
        pos: Point {
            x: 2 * start.x,
            y: start.y,
        },
        moves: moves.clone(),
    };

    let start = Instant::now();
    let gps = sum_of_gps(&mut warehouse, &mut robot);
    let duration = start.elapsed();
    println!("Part1: {} | {}s", gps, duration.as_secs_f32());

    let start = Instant::now();
    let gps = sum_of_gps(&mut warehouse_p2, &mut robot_p2);
    let duration = start.elapsed();
    println!("Part2: {} | {}s", gps, duration.as_secs_f32());

    Ok(())
}
