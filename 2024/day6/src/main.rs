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

fn check_loop(
    map: &Vec<Vec<char>>,
    v_map: &Vec<Vec<Vec<bool>>>,
    vis: &Vec<Vec<u8>>,
    (cur_y, cur_x): (i32, i32),
    dir: Direction,
) -> bool {
    let mut y = cur_y + dir.y;
    let mut x = cur_x + dir.x;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; map.len()]; map[0].len()];

    //check if a block could be placed
    if y >= 0 && y < v_map.len() as i32 && x >= 0 && x < v_map[0].len() as i32 {
        //obstacle could be placed, we turn
        if let Some(mut t_dir) = turn_right(dir) {
            //check if in this direction a location was already visited in the same direction
            y = cur_y + t_dir.y;
            x = cur_x + t_dir.x;

            //println!("first step: {y},{x} -> {t_dir}");

            //exit condition is we run out of the map or reach in internal loop
            while y >= 0 && y < v_map.len() as i32 && x >= 0 && x < v_map[0].len() as i32 {
                //we have already visited here so we have looped and did not find a direction match -> abort
                if visited[y as usize][x as usize] {
                    return false;
                }

                visited[y as usize][x as usize] = true;

                if map[y as usize][x as usize] != '#' {             
                    if vis[y as usize][x as usize] == 1 {
                        //println!("Visited: {y},{x} -> {t_dir}");
                        //location was visited -> check if the direction matches
                        if let Some(dir_pos) = get_dir_pos(t_dir) {
                            //println!("Check Pos: {dir_pos}:{t_dir}");
                            if v_map[y as usize][x as usize][dir_pos] {
                                return true;
                            }
                        }
                    }
                } else {
                    //we ran into a block -> take a step back then turn
                    //println!("turn right");
                    y = y - t_dir.y;
                    x = x - t_dir.x;
                    if let Some(td) = turn_right(t_dir) {
                        t_dir = td;
                    }
                }
                y = y + t_dir.y;
                x = x + t_dir.x;
                //println!("next step: {y},{x} -> {t_dir}");
            }
        }
    }
    false
}

fn get_visited(map: &Vec<Vec<char>>) -> (i32, usize) {
    let mut visited = 0;
    let mut vis: Vec<Vec<u8>> = vec![vec![0; map.len()]; map[0].len()];

    //maps which directions have been visited
    let mut v_map: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 4]; map.len()]; map[0].len()];
    let mut block_locations: Vec<(i32, i32, Direction)> = Vec::new();

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

                        //println!("Check Loop: {y},{x} -> {dir}");
                        let ploop = check_loop(map, &v_map, &vis, (y, x), dir);
                        if ploop {
                            //loop is potentially possible -> check if it is unique
                            let y_block = y + dir.y;
                            let x_block = x + dir.x;
                            if !block_locations.contains(&(y_block, x_block, dir)) {
                                //unique -> add it
                                block_locations.push((y_block, x_block, dir));
                                //println!("Block: {y_block}, {x_block} -> {dir}");
                            }
                        }

                        if let Some(dir_pos) = get_dir_pos(dir) {
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
