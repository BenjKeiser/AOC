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

fn get_steps_concat(stop: u64, operands: &VecDeque<u64>, mut cur: u64, op: OPS) -> bool {
    let mut oprnds = operands.clone();

    if let Some(val) = oprnds.pop_front() {
        if op == OPS::MULT {
            cur = cur * val;
        } else if op == OPS::ADD {
            cur = cur + val;
        } else if op == OPS::CONC {
            let mut lh = cur.to_string();
            let rh = val.to_string();
            lh.push_str(&rh);
            if let Ok(res) = lh.parse::<u64>() {
                cur = res;
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else {
        if cur == stop {
            return true;
        }
    }

    if operands.is_empty() {
        if cur == stop {
            return true;
        }
    } else if cur <= stop {
        if get_steps_concat(stop, &oprnds, cur, OPS::MULT) {
            return true;
        } else {
            if get_steps_concat(stop, &oprnds, cur, OPS::ADD) {
                return true;
            } else {
                if get_steps_concat(stop, &oprnds, cur, OPS::CONC) {
                    return true;
                }
            }
        }
    } else {
        return false;
    }

    false
}

fn get_steps(stop: u64, operands: &VecDeque<u64>, mut cur: u64, op: OPS) -> bool {
    let mut oprnds = operands.clone();
    
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
            return true;
        }
    }

    if operands.is_empty() {
        if cur == stop {
            return true;
        }
    } else if cur <= stop {
        if get_steps(stop, &oprnds, cur, OPS::MULT) {
            return true;
        } else {
            if get_steps(stop, &oprnds, cur, OPS::ADD) {
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
        let mut operands: VecDeque<u64> = eq.nbs.clone();
        if let Some(cur) = operands.pop_front() {
            if get_steps_concat(eq.res, &operands, cur, OPS::MULT) {
                sum += eq.res;
            } else {
                if get_steps_concat(eq.res, &operands, cur, OPS::ADD) {
                    sum += eq.res;
                } else {
                    if get_steps_concat(eq.res, &operands, cur, OPS::CONC) {
                        sum += eq.res;
                    }
                }
            }
        }
    }
    sum
}

fn verify_equations(eqs: &Vec<Equation>) -> u64 {
    let mut sum = 0;
    for eq in eqs {
        let mut operands: VecDeque<u64> = eq.nbs.clone();
        if let Some(cur) = operands.pop_front() {
            if get_steps(eq.res, &operands, cur, OPS::MULT) {
                sum += eq.res;
            } else {
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
    let verified = verify_equations_concat(&equations);
    println!("part2: {verified}");
    Ok(())
}
//1711 for part2
