use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;

#[derive(Clone, Copy, PartialEq)]
pub struct Direction {
    x: i32,
    y: i32,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

static DIRECTIONS: &'static [Direction] = &[
    Direction { x: 0, y: -1 },
    Direction { x: 1, y: 0 },
    Direction { x: 0, y: 1 },
    Direction { x: -1, y: 0 },
];

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn get_dir_pos(dir: Direction) -> Option<usize> {
    DIRECTIONS.iter().position(|x| x.y == dir.y && x.x == dir.x)
}

fn turn_right(dir: Direction) -> Option<Direction> {
    if let Some(mut pos) = get_dir_pos(dir) {
        if pos < DIRECTIONS.len() - 1 {
            pos += 1;
        } else {
            pos = 0;
        }
        return Some(DIRECTIONS[pos]);
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

    if y >= 0 && y < map.len() as i32 && x >= 0 && x < map[0].len() as i32 {
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

fn check_loop(map_orig: &Vec<Vec<char>>, (cur_y, cur_x): (i32, i32), dir: Direction) -> bool {
    let y = cur_y + dir.y;
    let x = cur_x + dir.x;
    let mut map = map_orig.clone();
    let mut visited: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; map[0].len()]; map.len()];

    //check if a block could be placed
    if y >= 0 && y < map.len() as i32 && x >= 0 && x < map[0].len() as i32 {
        if map[y as usize][x as usize] == '#' {
            //there already is a block, we can abort
            return false;
        }
    } else {
        //No block can be placed we exit
        return false;
    }

    //we place the block
    map[y as usize][x as usize] = '#';

    //Find Start position
    for i in 0..map.len() {
        if let Some(pos) = map[i].iter().position(|&x| x == '^') {
            let mut y = i as i32;
            let mut x = pos as i32;
            let mut t_dir = Direction { x: 0, y: -1 };
            if let Some(dir_pos) = get_dir_pos(t_dir) {
                visited[y as usize][x as usize][dir_pos] = true;
            }
            //println!("{y},{x} -> {dir}");
            loop {
                match get_next(&map, (y, x), t_dir) {
                    Some(((y_n, x_n), dir_n)) => {
                        y = y_n;
                        x = x_n;
                        t_dir = dir_n;

                        //check if we loop
                        if let Some(dir_pos) = get_dir_pos(t_dir) {
                            if visited[y as usize][x as usize][dir_pos] {
                                return true;
                            }
                            visited[y as usize][x as usize][dir_pos] = true;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }

    false
}

fn get_visited(map: &Vec<Vec<char>>) -> (i32, usize) {
    let mut visited = 0;
    let mut vis: Vec<Vec<u8>> = vec![vec![0; map[0].len()]; map.len()];

    //maps which directions have been visited
    let mut v_map: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
    let mut block_locations: Vec<(i32, i32)> = Vec::new();

    //Find Start position
    for i in 0..map.len() {
        if let Some(pos) = map[i].iter().position(|&x| x == '^') {
            let mut y = i as i32;
            let mut x = pos as i32;
            let mut dir = Direction { x: 0, y: -1 };
            vis[y as usize][x as usize] = 1;
            if let Some(dir_pos) = get_dir_pos(dir) {
                v_map[y as usize][x as usize][dir_pos] = true;
            }
            //println!("{y},{x} -> {dir}");
            loop {
                match get_next(map, (y, x), dir) {
                    Some(((y_n, x_n), dir_n)) => {
                        y = y_n;
                        x = x_n;
                        dir = dir_n;

                        vis[y as usize][x as usize] = 1;

                        if let Some(dir_pos) = get_dir_pos(dir) {
                            if !v_map[y as usize][x as usize][dir_pos] {
                                //println!("Check Loop: {y},{x} -> {dir}");
                                let ploop = check_loop(map, (y, x), dir);
                                if ploop {
                                    //loop is potentially possible -> check if it is unique
                                    let y_block = y + dir.y;
                                    let x_block = x + dir.x;
                                    if !block_locations.contains(&(y_block, x_block))
                                        && map[y_block as usize][x_block as usize] != '#'
                                    {
                                        //unique -> add it
                                        block_locations.push((y_block, x_block));
                                        //println!("Block: {y_block}, {x_block} -> {dir}");
                                    }
                                }
                            }

                            //println!("Set Visited: {y},{x} -> {dir_pos}:{dir}");
                            v_map[y as usize][x as usize][dir_pos] = true;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }

    //println!("{:?}", vis);

    for i in 0..vis.len() {
        let s: u8 = vis[i].iter().sum();
        visited += s as i32;
    }

    //println!("{:?}", vis);

    (visited, block_locations.len())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in fs::read_to_string(file_path)?.lines() {
        map.push(line.chars().collect());
    }
    let (visited, blocks) = get_visited(&map);
    println!("part1: {visited}");
    println!("part2: {blocks}");
    Ok(())
}
//1711 for part2
