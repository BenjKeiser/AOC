use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;

#[derive(Eq, Hash, Clone, Copy, PartialEq)]
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

fn get_score(map: &Vec<Vec<u8>>, trailhead: &(usize, usize)) -> usize {
    let mut steps: Vec<((usize, usize), Direction)> = Vec::new();
    let mut tops: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: HashSet<((usize, usize), Direction)> = HashSet::new();

    for dir in DIRECTIONS {
        steps.push((*trailhead, *dir));
    }

    while let Some(((y, x), dir)) = steps.pop() {
        let cur_step = map[y][x];

        //Skip already visited
        if visited.contains(&((y, x), dir)) {
            continue;
        }

        visited.insert(((y, x), dir));

        //println!("Check: {y}, {x} -> {dir}");

        if cur_step == 9 {
            if !tops.contains(&(y, x)) {
                tops.insert((y, x));
            }
        } else {
            let step_x: i32 = x as i32 + dir.x;
            let step_y: i32 = y as i32 + dir.y;

            //step out of bound
            if step_y < 0
                || step_y >= map.len() as i32
                || step_x < 0
                || step_x >= map[0].len() as i32
            {
                //println!("Out of bound: {step_y}, {step_x}");
                continue;
            }

            // step is invalid -> we no longer analize
            let next_step = map[step_y as usize][step_x as usize];
            if next_step != cur_step + 1 {
                //println!("Invalid {next_step} != {}", cur_step + 1);
                continue;
            }

            for n_dir in DIRECTIONS {
                if !(dir.x == -n_dir.x && dir.y == -n_dir.y) {
                    //println!("Add: {step_y}, {step_x} -> {n_dir}");
                    //skip going back
                    steps.push(((step_y as usize, step_x as usize), *n_dir));
                }
            }
        }
    }

    tops.len()
}

fn analyze_map(map: &Vec<Vec<u8>>, trailheads: &Vec<(usize, usize)>) -> usize {
    let mut total_score = 0;

    for th in trailheads {
        total_score += get_score(map, th);
    }

    total_score
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    const RADIX: u32 = 10;
    let mut map: Vec<Vec<u8>> = Vec::new();
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    let mut y: usize = 0;
    for line in fs::read_to_string(file_path)?.lines() {
        let mut row: Vec<u8> = Vec::new();
        let mut x: usize = 0;
        for c in line.chars() {
            if let Some(d) = c.to_digit(RADIX) {
                row.push(d as u8);
                if d == 0 {
                    trailheads.push((y, x));
                }
            } else {
                return Err("How did we get here: {c}".into());
            }
            x += 1;
        }
        map.push(row);
        y += 1;
    }
    println!("{:?}", map);

    let score = analyze_map(&map, &trailheads);
    println!("part1: {score}");

    Ok(())
}
