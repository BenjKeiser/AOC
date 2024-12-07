use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;

#[derive(Clone, PartialEq)]
pub struct Equation {
    res: u64,
    nbs: VecDeque<u64>,
}
#[derive(PartialEq, Debug)]
enum OPS {
    MULT,
    ADD,
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {:?})", self.res, self.nbs)
    }
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn get_steps(stop: u64, operands: &VecDeque<u64>, mut cur: u64, op: OPS) -> bool {
    let mut oprnds = operands.clone();

    println!("{stop}: {cur} ->{:?} {:?}", op, operands);

    if let Some(val) = oprnds.pop_front() {
        if op == OPS::MULT {
            cur = cur * val;
        } else if op == OPS::ADD {
            cur = cur + val;
        } else {
            return false;
        }
    } else {
        if cur == stop {
            println!("1 -> {cur} == {stop}");
            return true;
        }
    }

    if operands.is_empty() {
        if cur == stop {
            println!("1 -> {cur} == {stop}");
            return true;
        }
    } else if cur <= stop {
        if get_steps(stop, &oprnds, cur, OPS::MULT) {
            return true;
        }
        if get_steps(stop, &oprnds, cur, OPS::ADD) {
            return true;
        }
    } else {
        return false;
    }

    false
}

fn verify_equations(eqs: &Vec<Equation>) -> u64 {
    let mut sum = 0;
    for eq in eqs {
        let mut operands: VecDeque<u64> = eq.nbs.clone();
        if let Some(cur) = operands.pop_front() {
            if get_steps(eq.res, &operands, cur, OPS::MULT) {
                sum += eq.res;
            }
            else {
                if get_steps(eq.res, &operands, cur, OPS::ADD) {
                    sum += eq.res;
                }
            }
        }
    }
    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut equations: Vec<Equation> = Vec::new();
    for line in fs::read_to_string(file_path)?.lines() {
        if let Some((result, numbers)) = line.split_once(":") {
            let mut nbs: VecDeque<u64> = VecDeque::new();
            for n in numbers.split(' ') {
                if let Ok(num) = n.parse::<u64>() {
                    nbs.push_back(num);
                }
            }
            equations.push(Equation {
                res: result.parse::<u64>()?,
                nbs: nbs,
            });
        }
    }

    let verified = verify_equations(&equations);
    println!("part1: {verified}");
    //println!("part2: {blocks}");
    Ok(())
}
//1711 for part2
