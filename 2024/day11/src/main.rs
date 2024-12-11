use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::time::Instant;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn get_digits(stone: u64) -> u32 {
    if stone == 0 {
        1
    } else {
        (stone as f64).log10().floor() as u32 + 1
    }
}

fn do_blink(stone: u64) -> Vec<u64> {
    let mut stones: Vec<u64> = Vec::new();
    if stone == 0 {
        //stone is 0 new stone is 1
        stones.push(1);
    } else if get_digits(stone) % 2 == 0 {
        let s = stone.to_string();
        stones.push(s[..s.len() / 2].parse::<u64>().unwrap());
        stones.push(s[s.len() / 2..].parse::<u64>().unwrap());
    } else {
        stones.push(stone * 2024);
    }

    stones
}

fn get_nb_of_stones(stones: &[u64], blinks: u32) -> u64 {
    let mut stb: Vec<(u64, u32)> = Vec::new();
    let mut all_stones = 0;

    for stone in stones {
        stb.push((*stone, blinks));
        while !stb.is_empty() {
            if let Some((st, bl)) = stb.pop() {
                if bl == 0 {
                    all_stones += 1;
                } else {
                    let sts = do_blink(st);
                    for s in sts {
                        stb.push((s, bl - 1));
                    }
                }
            }
        }
    }

    all_stones
}

fn get_nb_of_stones_rec(stone: u64, blinks: u32, stone_map: &mut HashMap<(u64, u32), u64>) -> u64 {
    let mut all_stones = 0;

    let sts = do_blink(stone);
    if blinks == 1 {
        return sts.len() as u64;
    }
    else {
        for s in sts {
            if let Some(val) = stone_map.get(&(s, blinks-1)) {
                all_stones += *val;
            }
            else {
                let stones = get_nb_of_stones_rec(s, blinks-1, stone_map);
                stone_map.insert((s, blinks-1), stones);
                all_stones += stones;                
            }
        }    
        
    }    

    all_stones
}

fn solve_part2(stones: &[u64], blinks: u32) -> u64 {
    let mut all_stones = 0;
    let mut stone_map: HashMap<(u64, u32), u64> = HashMap::new();
    for stone in stones {
        all_stones += get_nb_of_stones_rec(*stone, blinks, &mut stone_map);
    }
    all_stones
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut stones: Vec<_> = Vec::new();
    for line in fs::read_to_string(file_path)?.lines() {
        for nb in line.split(' ') {
            let stone = nb.parse::<u64>().unwrap();
            stones.push(stone);
        }
    }

    let start = Instant::now();
    let nb_stones = get_nb_of_stones(&stones, 25);
    let duration = start.elapsed();
    println!("Part1: {nb_stones} | {}s", duration.as_secs_f32());
    let start = Instant::now();
    let nb_stones = solve_part2(&stones, 75);
    let duration = start.elapsed();
    println!("Part2: {nb_stones} | {}s", duration.as_secs_f32());

    Ok(())
}
