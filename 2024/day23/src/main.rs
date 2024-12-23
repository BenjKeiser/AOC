use regex::Regex;
use std::collections::{HashSet, HashMap};
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

fn get_all_connections(connections: &Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut computers: HashMap<String, Vec<String>> = HashMap::new();

    for (first, second) in connections {
        computers
            .entry(first.clone())
            .and_modify(|con| con.push(second.clone()))
            .or_insert_with(|| vec![second.clone()]);
        computers
            .entry(second.clone())
            .and_modify(|con| con.push(first.clone()))
            .or_insert_with(|| vec![first.clone()]);
    }
    computers
}

fn get_three_interconnects(computers: &HashMap<String, Vec<String>>, start: char) -> HashSet<Vec<String>> {
    let mut threes: HashSet<Vec<String>> = HashSet::new();


    for (c1, cs) in computers {
        if !c1.starts_with(start) {
            continue;
        }
        for k in 0..cs.len() - 1 {
            for y in k + 1..cs.len() {
                let c2 = &cs[k];
                let c3 = &cs[y];

                if let Some(c2_c) = computers.get(c2) {
                    if let Some(c3_c) = computers.get(c3) {
                        if c2_c.contains(&c1)
                            && c2_c.contains(&c3)
                            && c3_c.contains(&c1)
                            && c3_c.contains(&c2)
                        {
                            let mut v2 = vec![c1.to_string(), c2.to_string(), c3.to_string()];
                            v2.sort();
                            threes.insert(v2);
                        }
                    }
                }
            }
        }
    }

    threes
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut connections: Vec<(String, String)> = Vec::new();

    let re = Regex::new(r"^(.+)-(.+)$").unwrap();

    for line in fs::read_to_string(file_path)?.lines() {
        if let Some(captures) = re.captures(line) {
            let first = captures.get(1).map_or("", |m| m.as_str());
            let second = captures.get(2).map_or("", |m| m.as_str());
            connections.push((first.to_string(), second.to_string()));
        }
    }

    let start = Instant::now();
    let computers = get_all_connections(&connections);
    let three_interconnects = get_three_interconnects(&computers, 't');
    let threes = three_interconnects.len();
    let duration = start.elapsed();
    println!(
        "Part1: {threes} | {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();
    let bananas = 0;
    let duration = start.elapsed();
    println!(
        "Part2: {2} | {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
