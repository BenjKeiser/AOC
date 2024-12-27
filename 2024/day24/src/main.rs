use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::time::Instant;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

#[derive(Eq, Hash, Clone, Copy, PartialEq, Debug)]
pub enum Op {
    AND,
    OR,
    XOR,
    UNKNOWN,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Op::AND => write!(f, "AND"),
            Op::OR => write!(f, "OR"),
            Op::XOR => write!(f, "XOR"),
            _ => write!(f, "UNKNOWN"),
        }
    }
}

impl Op {
    fn to_op(str: &str) -> Op {
        match str {
            "AND" => Op::AND,
            "OR" => Op::OR,
            "XOR" => Op::XOR,
            _ => Op::UNKNOWN,
        }
    }
}

#[derive(Eq, Hash, Clone, Debug, PartialEq)]
pub struct Equation {
    nb1: String,
    nb2: String,
    op: Op,
}

impl Equation {
    fn solve(self: &Self, values: &BTreeMap<String, u8>) -> Option<u8> {
        let mut res: u8 = 0;
        if let Some(&n1) = values.get(&self.nb1) {
            if let Some(&n2) = values.get(&self.nb2) {
                match self.op {
                    Op::AND => {
                        res = n1 & n2;
                    }
                    Op::OR => {
                        res = n1 | n2;
                    }
                    Op::XOR => {
                        res = n1 ^ n2;
                    }
                    _ => {}
                }
                return Some(res);
            }
        }
        None
    }
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.nb1, self.op, self.nb2)
    }
}

fn get_z(values: &mut BTreeMap<String, u8>, equations: &Vec<(String, Equation)>) -> usize {
    let mut z = 0;
    let mut eqs_copy = equations.clone();

    while !eqs_copy.is_empty() {
        let eqs = eqs_copy.clone();
        for (idx, (value, eq)) in eqs.iter().rev().enumerate() {
            if let Some(val) = eq.solve(values) {
                values.insert(value.to_string(), val);
                eqs_copy.remove(eqs.len() - 1 - idx);
            }
        }
    }

    for (idx, (_k, v)) in values.iter().filter(|(k, _)| k.starts_with('z')).enumerate(){
       z += (*v as usize) << idx;
    }

    z
}

fn get_switched(values: &BTreeMap<String, u8>, equations: &Vec<(String, Equation)>) -> Vec<String> {
    let mut switched = Vec::new();
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    let mut max_len = 0;

    for (idx, (_k, v)) in values.iter().filter(|(k, _)| k.starts_with('x')).enumerate(){
        x += (*v as usize) << idx;
     }

     for (idx, (_k, v)) in values.iter().filter(|(k, _)| k.starts_with('y')).enumerate(){
        y += (*v as usize) << idx;
     }

     for (idx, (_k, v)) in values.iter().filter(|(k, _)| k.starts_with('z')).enumerate(){
        z += (*v as usize) << idx;
        max_len = idx;
     }

     //let x_y = x & y; // <- test_input_p2
     let x_y = x + y; // <- real system

     println!("x:       {}", x);
     println!("y:       {}", y);
     println!("x + y:   {}", x_y);
     println!("z:       {}", z);
     println!("diff:    {}", x_y ^ z);

     for i in 0..=max_len {
        if (((x_y ^ z) >> i) & 1) == 1 {
            let mismatch = format!("z{:02}", i);
            if ((z >> i) & 1) == 1 {
                println!("z is 1, should be 0");
            } 
            else {
                println!("z is 0, should be 1");
            }
            println!("mismatch {mismatch}");
        }
     }

     switched
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut values = BTreeMap::new();
    let mut equations = Vec::new();

    for line in fs::read_to_string(file_path)?.lines() {
        if line.contains(':') {
            let str: Vec<&str> = line.trim().splitn(2, ':').collect();
            values.insert(str[0].to_string(), str[1].trim().parse::<u8>()?);
        }

        if line.contains("->") {
            let str: Vec<&str> = line.splitn(2, " -> ").collect();
            let eq: Vec<&str> = str[0].splitn(3, ' ').collect();
            equations.push((
                str[1].to_string(),
                Equation {
                    nb1: eq[0].to_string(),
                    nb2: eq[2].to_string(),
                    op: Op::to_op(eq[1]),
                },
            ));
        }
    }

    let start = Instant::now();

    let z = get_z(&mut values, &equations);

    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        z,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();

    let _switched = get_switched(&values, &equations);

    let duration = start.elapsed();

    println!(
        "Part2: {} | {}s {}ms {}µs {}ns",
        2,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
