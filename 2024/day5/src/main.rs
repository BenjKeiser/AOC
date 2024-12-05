use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;

pub struct Rule {
    before: i32,
    after: i32,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.before, self.after)
    }
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn correct_update(rules: &Vec<Rule>, update: &Vec<i32>) -> u64 {
    let mut val = 0;
    let mut u = update.clone();
    let mut correct = true;

    for r in rules {
        if let Some(pos_b) = u.iter().position(|&x| x == r.before) {
            if let Some(pos_a) = u.iter().position(|&x| x == r.after) {
                if pos_a < pos_b {
                    u.swap(pos_a, pos_b);
                    val = correct_update(rules, &u);
                    correct = false;
                    break;
                }
            }
        }
    }
    if correct {
        val = u[u.len() / 2] as u64
    }

    val
}

fn verify_updates(rules: &Vec<Rule>, updates: &Vec<Vec<i32>>) -> (u64, u64) {
    let mut sumc: u64 = 0;
    let mut sumic: u64 = 0;

    for u in updates {
        let mut correct: bool = true;
        for r in rules {
            if let Some(pos_b) = u.iter().position(|&x| x == r.before) {
                if let Some(pos_a) = u.iter().position(|&x| x == r.after) {
                    if pos_a < pos_b {
                        //after value is before -> we violated a rule
                        //println!("{:?}", u);
                        //println!("{} -> {:?} < {:?}", r, pos_a, pos_b);
                        correct = false;

                        sumic += correct_update(rules, u);
                        break;
                    }
                }
            }
        }
        if correct {
            sumc += u[u.len() / 2] as u64;
        }
    }

    (sumc, sumic)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut rules: Vec<Rule> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();
    for line in fs::read_to_string(file_path)?.lines() {
        match line.split_once("|") {
            Some((b, a)) => {
                rules.push(Rule {
                    before: b.parse::<i32>()?,
                    after: a.parse::<i32>()?,
                });
            }
            None => {
                if line.len() > 0 {
                    let mut upd: Vec<i32> = Vec::new();
                    for nb in line.split(",") {
                        upd.push(nb.parse::<i32>()?);
                    }
                    updates.push(upd);
                }
            }
        }
    }
    let (correct, incorrect) = verify_updates(&rules, &updates);
    println!("part1: {correct}");
    println!("part2: {incorrect}");
    Ok(())
}
