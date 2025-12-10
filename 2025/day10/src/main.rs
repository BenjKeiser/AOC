use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::ffi::OsString;
use std::time::Instant;
use std::{env, fs};

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Indicator {
    Off,
    On,
}

impl Indicator {
    fn toggle(self: &mut Self) {
        if *self == Indicator::On {
            *self = Indicator::Off;
        } else if *self == Indicator::Off {
            *self = Indicator::On;
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Machine {
    i: Vec<Indicator>,
    b: Vec<Vec<u64>>,
    j: Vec<u64>,
}

fn bfs_recursive(
    m: &Machine,
    queue: &mut VecDeque<(Vec<Indicator>, u64)>,
    visited: &mut HashSet<Vec<Indicator>>,
) -> u64 {
    if let Some((node, cost)) = queue.pop_front() {
        if !visited.contains(&node) {

            if node == m.i {
                return cost;
            }
            
            // insert next
            for buttons in m.b.iter() {
                let mut next = node.clone();
                for b in buttons {
                    next[*b as usize].toggle();
                }
                queue.push_back((next, cost+1));
            }
            
            visited.insert(node);
        }
        return bfs_recursive(m, queue, visited);
    }

    u64::MAX
}

fn turn_on_machine(machine: &Machine) -> u64 {
    let start = vec![Indicator::Off; machine.i.len()];

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start, 0));

    bfs_recursive(machine, &mut queue, &mut visited)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let start = Instant::now();

    let mut machines: Vec<Machine> = Vec::new();

    for line in fs::read_to_string(file_path)?.lines() {
        let mut m: Machine = Machine {
            i: Vec::new(),
            b: Vec::new(),
            j: Vec::new(),
        };

        //Parse Indicator
        if let Some(idx) = line.find(']') {
            for c in line.chars().skip(1).take(idx - 1) {
                if c == '.' {
                    m.i.push(Indicator::Off);
                } else if c == '#' {
                    m.i.push(Indicator::On);
                }
            }
        }

        //buttons
        for b in line.split('(').skip(1) {
            let b_str = b.split(')').next().unwrap();
            let buttons: Vec<u64> = b_str
                .split(',')
                .filter_map(|s| s.trim().parse::<u64>().ok())
                .collect();
            m.b.push(buttons);
        }

        //joltage
        if let Some(j) = line.split('{').skip(1).next() {
            let j_str = j.split('}').next().unwrap();
            m.j = j_str
                .split(',')
                .filter_map(|s| s.trim().parse::<u64>().ok())
                .collect();
        }

        machines.push(m);
    }

    let duration = start.elapsed();

    println!(
        "Parse: {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();

    let buttons: u64 = machines.iter().map(|m| turn_on_machine(m)).sum();

    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        buttons,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();

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
