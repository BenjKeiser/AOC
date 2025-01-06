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

    for (idx, (_k, v)) in values
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .enumerate()
    {
        z += (*v as usize) << idx;
    }

    z
}

fn get_equation(value: &str, equations: &Vec<(String, Equation)>) -> Option<Equation> {
    for (v, e) in equations {
        if *v == *value {
            return Some(e.clone());
        }
    }
    None
}

fn expand_equation(value: &str, equations: &Vec<(String, Equation)>) -> String {
    let nb1;
    let nb2;
    if let Some(eq) = get_equation(value, equations) {
        if !eq.nb1.starts_with("x")
            && !eq.nb1.starts_with("y")
            && !eq.nb1.starts_with("z")
            && !eq.nb1.starts_with("carry")
        {
            nb1 = expand_equation(&eq.nb1, equations);
        } else {
            nb1 = eq.nb1;
        }
        if !eq.nb2.starts_with("x")
            && !eq.nb2.starts_with("y")
            && !eq.nb2.starts_with("z")
            && !eq.nb2.starts_with("carry")
        {
            nb2 = expand_equation(&eq.nb2, equations);
        } else {
            nb2 = eq.nb2;
        }
        return format!("({} {} {})", nb1, eq.op, nb2);
    }
    "ERROR".to_string()
}

fn remap_equations(
    equations: &Vec<(String, Equation)>,
    old: &str,
    new: &str,
) -> Vec<(String, Equation)> {
    let mut remap: Vec<(String, Equation)> = Vec::new();

    for (v, eq) in equations {
        if v == old {
            remap.push((new.to_string(), eq.clone()));
        } else if eq.nb1 == *old {
            remap.push((
                v.clone(),
                Equation {
                    nb1: new.to_string(),
                    nb2: eq.nb2.clone(),
                    op: eq.op,
                },
            ));
        } else if eq.nb2 == *old {
            remap.push((
                v.clone(),
                Equation {
                    nb1: eq.nb1.clone(),
                    nb2: new.to_string(),
                    op: eq.op,
                },
            ));
        } else {
            remap.push((v.clone(), eq.clone()));
        }
    }

    remap
}

fn find_carry(equations: &Vec<(String, Equation)>, z: &str) -> Option<String> {
    let index = &z[1..];
    let x = "x".to_string() + index;
    let y = "y".to_string() + index;

    if let Some(eq) = get_equation(z, equations) {
        // needs to be XOR
        if eq.op != Op::XOR {
            println!("NO XOR!");
            return None
        }

        // Check if nb1 is the value -> nb2 must be the carry
        if let Some(eq1) = get_equation(&eq.nb1, equations) {
            if eq1.op == Op::XOR
                && ((eq1.nb1 == x && eq1.nb2 == y) || (eq1.nb1 == y && eq1.nb2 == x))
            {
                return Some(eq.nb2);
            }
        }

        // Check if nb2 is the value -> nb1 must be the carry
        if let Some(eq2) = get_equation(&eq.nb2, equations) {
            if eq2.op == Op::XOR
                && ((eq2.nb1 == x && eq2.nb2 == y) || (eq2.nb1 == y && eq2.nb2 == x))
            {
                return Some(eq.nb1);
            }
        }
    }

    None
}

fn verify_carry(equations: &Vec<(String, Equation)>, z: &str, carry: &str) -> bool {
    //Carryn = (Xn AND Yn) OR (Carryn-1 AND (Xn XOR Yn))

    let index = &z[1..];
    let c_idx = index.parse::<usize>().unwrap() - 1;
    let old_carry = format!("carry{:02}", c_idx-1);
    let x = format!("x{:02}", c_idx);
    let y = format!("y{:02}", c_idx);

    let r_str = expand_equation(carry, equations);
    if c_idx == 0 {
        let c_str = format!("({x} AND {y})");
        if c_str == r_str
        {
            return true
        }
        else {
            println!("Mismatch in Carry match: ");
            println!("Expected String: {c_str}");
            println!("Result String: {}", r_str);
        }
    }
    else {
        let mut r_permutations = Vec::new();

        let c_str = format!("(({y} AND {x}) OR ({old_carry} AND ({x} XOR {y})))");
        r_permutations.push(c_str);
        let c_str = format!("(({y} AND {x}) OR ({old_carry} AND ({y} XOR {x})))");
        r_permutations.push(c_str);
        let c_str = format!("(({x} AND {y}) OR ({old_carry} AND ({x} XOR {y})))");
        r_permutations.push(c_str);
        let c_str = format!("(({x} AND {y}) OR ({old_carry} AND ({y} XOR {x})))");
        r_permutations.push(c_str);
        let c_str = format!("(({x} AND {y}) OR (({y} XOR {x}) AND {old_carry}))");
        r_permutations.push(c_str);
        let c_str = format!("(({x} AND {y}) OR (({x} XOR {y}) AND {old_carry}))");
        r_permutations.push(c_str);
        let c_str = format!("(({y} AND {x}) OR (({y} XOR {x}) AND {old_carry}))");
        r_permutations.push(c_str);
        let c_str = format!("(({y} AND {x}) OR (({x} XOR {y}) AND {old_carry}))");
        r_permutations.push(c_str);
        let c_str = format!("((({x} XOR {y}) AND {old_carry}) OR ({y} AND {x}))");
        r_permutations.push(c_str);
        let c_str = format!("((({x} XOR {y}) AND {old_carry}) OR ({x} AND {y}))");
        r_permutations.push(c_str);
        let c_str = format!("((({y} XOR {x}) AND {old_carry}) OR ({x} AND {y}))");
        r_permutations.push(c_str);
        let c_str = format!("((({y} XOR {x}) AND {old_carry}) OR ({y} AND {x}))");
        r_permutations.push(c_str);
        let c_str = format!("(({old_carry} AND ({y} XOR {x})) OR ({y} AND {x}))");
        r_permutations.push(c_str);
        let c_str = format!("(({old_carry} AND ({y} XOR {x})) OR ({x} AND {y}))");
        r_permutations.push(c_str);
        let c_str = format!("(({old_carry} AND ({x} XOR {y})) OR ({y} AND {x}))");
        r_permutations.push(c_str);
        let c_str = format!("(({old_carry} AND ({x} XOR {y})) OR ({x} AND {y}))");
        r_permutations.push(c_str);

        for c_str in &r_permutations {
            if *c_str == r_str
            {
                return true
            }
        }

        println!("Could not match carry:");
        println!("{r_str}");
        println!("Might be due to missing permutations:");
        for c_str in r_permutations {
            println!("{c_str}");
        }
    }

    false
}

fn get_switched(values: &BTreeMap<String, u8>, equations: &Vec<(String, Equation)>) -> Vec<String> {
    let mut switched = Vec::new();
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    let mut max_len = 0;

    for (idx, (_k, v)) in values
        .iter()
        .filter(|(k, _)| k.starts_with('x'))
        .enumerate()
    {
        x += (*v as usize) << idx;
    }

    for (idx, (_k, v)) in values
        .iter()
        .filter(|(k, _)| k.starts_with('y'))
        .enumerate()
    {
        y += (*v as usize) << idx;
    }

    for (idx, (_k, v)) in values
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .enumerate()
    {
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

    let mut eqs = equations.clone();
    //let mut eqs = remap_equations(equations, "mcg", "carry00");
    //eqs = remap_equations(&eqs, "pqk", "carry01");
    //eqs = remap_equations(&eqs, "gkk", "carry02");

    //z00 manually checked, is correct
    //z45 manually checked, is correct

    for i in 1..max_len {
        let z = format!("z{:02}", i);
        let c = format!("carry{:02}", i - 1);

        if let Some(carry) = find_carry(&eqs, &z) {
            println!("{c} -> {carry}");
            // we need to verify that the carry equation is correct -> for zn we verify carry(n-1)
            if verify_carry(&eqs, &z, &carry) {
                eqs = remap_equations(&eqs, &carry, &c);
                println!("{} => {}", z, expand_equation(&z, &eqs));
            } else {
                println!(
                    "Carry {} could not be verified for: {} => {}",
                    carry,
                    z,
                    expand_equation(&z, &eqs)
                );
                break;
            }
        } else {
            println!("No carry found for: {} => {}", z, expand_equation(&z, &eqs));
            break;
        }

        /*
        let mismatch = format!("z{:02}", i);
        if (((x_y ^ z) >> i) & 1) == 1 {
            if ((z >> i) & 1) == 1 {
                println!("z is 1, should be 0");
            } else {
                println!("z is 0, should be 1");
            }
            println!("mismatch {mismatch}");
        }
        */
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
        "Part2: dqr,dtk,pfw,shh,vgs,z21,z33,z39 | {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
