use regex::Regex;
use std::collections::{HashMap, HashSet};
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

    computers.iter_mut().for_each(|(_k, v)| v.sort());

    computers
}

fn get_three_interconnects(
    computers: &HashMap<String, Vec<String>>,
    start: char,
) -> HashSet<Vec<String>> {
    let mut threes: HashSet<Vec<String>> = HashSet::new();

    for (c1, cs) in computers {
        if start != '0' && !c1.starts_with(start) {
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

fn get_largest_group(
    computers: &HashMap<String, Vec<String>>,
    all: &Vec<String>,
    groups: &HashSet<Vec<String>>,
) -> Vec<String> {
    let mut largest_group = Vec::new();
    let mut new_groups: HashSet<Vec<String>> = HashSet::new();

    for group in groups {
        for comp in all {
            if !group.contains(comp) {
                let mut add: bool = true;
                if let Some(c1) = computers.get(comp) {
                    for c in group {
                        if let Some(c2) = computers.get(c) {
                            if !c1.contains(c) || !c2.contains(comp) {
                                add = false;
                                break;
                            }
                        }
                    }
                }
                if add {
                    let mut new_group = group.clone();
                    new_group.push(comp.to_string());
                    new_group.sort();
                    new_groups.insert(new_group);
                }
            }
        }
    }

    let max_len = new_groups.iter().map(|vec| vec.len()).max().unwrap_or(0);
    let filtered_set: HashSet<Vec<String>> = new_groups
        .into_iter()
        .filter(|vec| vec.len() == max_len)
        .collect();

    if filtered_set.len() > 1 {
        largest_group = get_largest_group(computers, all, &filtered_set);
    } else {
        if let Some(l) = filtered_set.iter().next() {
            largest_group = l.clone();
        }
    }

    largest_group
}

fn get_longest_interconnect(computers: &HashMap<String, Vec<String>>) -> Vec<String> {
    let threes = get_three_interconnects(computers, '0');

    let all_computers: Vec<String> = computers.keys().cloned().collect();

    let interconnect = get_largest_group(computers, &all_computers, &threes);

    println!("{:?}", interconnect);

    interconnect
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
    let longest_interconnect = get_longest_interconnect(&computers);
    let mut result: String = String::from("NO OUTPUT");
    if let Some(r)= longest_interconnect.first() {
        result = r.to_string();
        for c in longest_interconnect.iter().skip(1) {
            result = result + ",";
            result.push_str(c);
        }

    }
    let duration = start.elapsed();

    println!(
        "Part2: {} | {}s {}ms {}µs {}ns",
        result,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
