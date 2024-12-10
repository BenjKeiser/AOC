use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;

#[derive(Clone, PartialEq)]
pub struct Equation {
    res: u64,
    nbs: Vec<u64>,
}
#[derive(PartialEq, Debug)]
enum OPS {
    MULT,
    ADD,
    CONC,
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

fn conc(mut lh: u64, rh: u64) -> u64 {
    if rh == 0 {
        return lh * 10;
    } else {
        let mut r: i64 = rh as i64;
        while r > 0 {
            lh *= 10;
            r /= 10;
        }
        return lh + rh;
    }
}

fn get_steps_concat(stop: u64, operands: &[u64], mut cur: u64, op: OPS) -> bool {
    //let mut oprnds = operands.clone();
    if !operands.is_empty() {
        let val = operands[0];
        if op == OPS::MULT {
            cur = cur * val;
        } else if op == OPS::ADD {
            cur = cur + val;
        } else if op == OPS::CONC {
            cur = conc(cur, val);
        } else {
            return false;
        }
    }

    //we checked the last element of operands
    if operands.len() == 1 {
        if cur == stop {
            return true;
        }
    } else if cur <= stop {
        if get_steps_concat(stop, &operands[1..], cur, OPS::MULT) {
            return true;
        } else {
            if get_steps_concat(stop, &operands[1..], cur, OPS::ADD) {
                return true;
            } else {
                if get_steps_concat(stop, &operands[1..], cur, OPS::CONC) {
                    return true;
                }
            }
        }
    } else {
        return false;
    }

    false
}

fn get_steps(stop: u64, operands: &[u64], mut cur: u64, op: OPS) -> bool {
    if !operands.is_empty() {
        let val = operands[0];
        if op == OPS::MULT {
            cur = cur * val;
        } else if op == OPS::ADD {
            cur = cur + val;
        } else {
            return false;
        }
    }

    if operands.len() == 1 {
        if cur == stop {
            return true;
        }
    } else if cur <= stop {
        if get_steps(stop, &operands[1..], cur, OPS::MULT) {
            return true;
        } else {
            if get_steps(stop, &operands[1..], cur, OPS::ADD) {
                return true;
            }
        }
    } else {
        return false;
    }

    false
}

fn verify_equations_concat(eqs: &Vec<Equation>) -> u64 {
    let mut sum = 0;
    for eq in eqs {
        let cur = eq.nbs[0];
        if get_steps_concat(eq.res, &eq.nbs[1..], cur, OPS::MULT) {
            sum += eq.res;
        } else {
            if get_steps_concat(eq.res, &eq.nbs[1..], cur, OPS::ADD) {
                sum += eq.res;
            } else {
                if get_steps_concat(eq.res, &eq.nbs[1..], cur, OPS::CONC) {
                    sum += eq.res;
                }
            }
        }
    }
    sum
}

fn verify_equations(eqs: &Vec<Equation>) -> u64 {
    let mut sum = 0;
    for eq in eqs {
        let cur = eq.nbs[0];
        if get_steps(eq.res, &eq.nbs[1..], cur, OPS::MULT) {
            sum += eq.res;
        } else {
            if get_steps(eq.res, &eq.nbs[1..], cur, OPS::ADD) {
                sum += eq.res;
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
            let mut nbs: Vec<u64> = Vec::new();
            for n in numbers.split(' ') {
                if let Ok(num) = n.parse::<u64>() {
                    nbs.push(num);
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
    let verified = verify_equations_concat(&equations);
    println!("part2: {verified}");
    Ok(())
}
