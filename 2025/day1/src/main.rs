use std::error::Error;
use std::ffi::OsString;
use std::{env, fmt, fs};

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum Turn {
    L = 0,
    R = 1,
}

impl TryFrom<char> for Turn {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Turn::L),
            'R' => Ok(Turn::R),
            _ => Err("Invalid OpCode"),
        }
    }
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Turn::L => write!(f, "L")?,
            Turn::R => write!(f, "R")?,
        }
        Ok(())
    }
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn turn_dial(pos: i32, turn: Turn, amount: i32) -> (i32, u32) {
    let mut new_pos;
    let mut clicks: u32 = amount as u32 / 100;
    match turn {
        Turn::L => new_pos = pos - (amount % 100),
        Turn::R => new_pos = pos + (amount % 100),
    }

    if new_pos < 0 {
        if pos != 0 {
            clicks += 1;
        }
        new_pos = 100 + new_pos;
    }

    if new_pos > 99 {
        if new_pos != 100 {
            clicks += 1;
        }
        new_pos = new_pos - 100;
    }

    return (new_pos, clicks);
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut turns: Vec<(Turn, i32)> = Vec::new();
    for line in fs::read_to_string(file_path)?.lines() {
        let mut chars = line.chars();

        if let Some(first) = chars.next() {
            let rest: String = chars.collect();

            let turn = rest.parse::<i32>()?;
            turns.push((Turn::try_from(first).unwrap(), turn));
        }
    }

    let mut zero_cnt: u32 = 0;
    let mut pos = 50;
    let mut clicks;
    let mut zero_cnt_2: u32 = 0;
    for t in turns.iter() {
        //let old_pos = pos;
        (pos, clicks) = turn_dial(pos, t.0, t.1);
        //println!("{} <- {} , {} t {}   | {} ", pos, t.0, old_pos, t.1, clicks);
        if pos == 0 {
            zero_cnt += 1;
            zero_cnt_2 += 1;
        }
        zero_cnt_2 += clicks;
    }
    println!("Part 1: {} zeroes", zero_cnt);
    println!("Part 2: {} zeroes", zero_cnt_2);


    Ok(())
}
