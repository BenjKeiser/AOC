use regex::Regex;
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Button {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Prize {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Machine {
    a: Button,
    b: Button,
    p: Prize,
}

impl Machine {
    fn solve(&self) -> Option<(usize, usize)> {
        let b: f64 = (self.a.x * self.p.y - self.a.y * self.p.x)
            / (self.a.x * self.b.y - self.b.x * self.a.y);
        let a: f64 = (self.p.y - (self.b.y* b)) / self.a.y;
        if a.fract() == 0.0 && b.fract() == 0.0 {
            return Some((a as usize, b as usize));
        }
        None
    }

    fn get_tokens(&self) -> Option<usize> {
        if let Some((a, b)) = self.solve() {
            return Some(a * 3 + b * 1);
        }

        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut machines: Vec<Machine> = Vec::new();
    let mut numbers: Vec<_> = Vec::new();

    // Regex to capture integers after X or Y markers
    let re = Regex::new(r"[XY][+=](\d+)").unwrap();

    for line in fs::read_to_string(file_path)?.lines() {
        for caps in re.captures_iter(line) {
            if let Some(num_str) = caps.get(1) {
                if let Ok(num) = num_str.as_str().parse::<i32>() {
                    numbers.push(num);
                }
            }
        }
    }

    if numbers.len() % 6 != 0 {
        return Err("Error while parsing input".into());
    }

    let mut idx = 0;
    while idx < numbers.len() {
        let b_a: Button = Button {
            x: numbers[idx] as f64,
            y: numbers[idx + 1] as f64,
        };
        idx += 2;
        let b_b: Button = Button {
            x: numbers[idx] as f64,
            y: numbers[idx + 1] as f64,
        };
        idx += 2;
        let prize: Prize = Prize {
            x: numbers[idx] as f64,
            y: numbers[idx + 1] as f64,
        };
        idx += 2;
        machines.push(Machine {
            a: b_a,
            b: b_b,
            p: prize,
        });
    }

    //println!("{:?}", machines);

    let start = Instant::now();
    let tokens: usize = machines
        .iter()
        .filter_map(|machine| machine.get_tokens())
        .sum();
    let duration = start.elapsed();
    println!("Part1: {tokens} | {}s", duration.as_secs_f32());

    for m in &mut machines {
        m.p.x += 10000000000000f64;
        m.p.y += 10000000000000f64;
    }

    let start = Instant::now();
    let tokens: usize = machines
        .iter()
        .filter_map(|machine| machine.get_tokens())
        .sum();
    let duration = start.elapsed();
    println!("Part2: {tokens} | {}s", duration.as_secs_f32());

    Ok(())
}
