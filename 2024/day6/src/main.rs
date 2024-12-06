use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;

#[derive(Clone, Copy)]
pub struct Direction {
    x: i32,
    y: i32,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn turn_right(dir: Direction) -> Option<Direction> {
    let dirs = [
        Direction { x: 0, y: -1 },
        Direction { x: 1, y: 0 },
        Direction { x: 0, y: 1 },
        Direction { x: -1, y: 0 },
    ];

    if let Some(mut pos) = dirs.iter().position(|x| x.y == dir.y && x.x == dir.x) {
        if pos < dirs.len() - 1 {
            pos += 1;
        } else {
            pos = 0;
        }
        return Some(dirs[pos]);
    }
    None
}

fn get_next(
    map: &Vec<Vec<char>>,
    (cur_y, cur_x): (i32, i32),
    dir: Direction,
) -> Option<((i32, i32), Direction)> {
    let mut direction: Direction = dir;

    let y = cur_y + direction.y;
    let x = cur_x + direction.x;

    if y >= 0 && y < map.len() as i32 && x >= 0 && y <= map[0].len() as i32 {
        if map[y as usize][x as usize] == '.' || map[y as usize][x as usize] == '^' {
            return Some(((y, x), direction));
        } else if map[y as usize][x as usize] == '#' {
            match turn_right(direction) {
                Some(d) => {
                    direction = d;
                    return Some(((cur_y, cur_x), direction));
                }
                None => {
                    return None;
                }
            }
        }
    } else {
        return None;
    }
    None
}

fn get_visited(map: &Vec<Vec<char>>) -> i32 {
    let mut visited = 0;
    let mut vis: Vec<Vec<u8>> = vec![vec![0; map.len()]; map[0].len()];

    //Find Start position
    for i in 0..map.len() {
        if let Some(pos) = map[i].iter().position(|&x| x == '^') {
            let mut y = i as i32;
            let mut x = pos as i32;
            let mut dir = Direction { x: 0, y: -1 };
            vis[y as usize][x as usize] = 1;
            //println!("{y},{x} -> {dir}");
            loop {
                match get_next(map, (y, x), dir) {
                    Some(((y_n, x_n), dir_n)) => {
                        y = y_n;
                        x = x_n;
                        dir = dir_n;
                        vis[y as usize][x as usize] = 1;
                        //println!("{y},{x} -> {dir}");
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }

    for i in 0..vis.len() {
        let s: u8 = vis[i].iter().sum();
        visited += s as i32;
    }

    //println!("{:?}", vis);

    visited
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in fs::read_to_string(file_path)?.lines() {
        map.push(line.chars().collect());
    }
    let visited = get_visited(&map);
    println!("part1: {visited}");
    Ok(())
}
