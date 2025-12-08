use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct JBox {
    x: u64,
    y: u64,
    z: u64,
}

impl JBox {
    fn distance_squared(&self, b: &JBox) -> u64 {
        (b.x - self.x).pow(2) + (b.y - self.y).pow(2) + (b.z - self.z).pow(2)
    }
}

fn connect_boxes(jbox: &Vec<JBox>, nb_conn: usize) -> u64 {
    let mut heap: BinaryHeap<Reverse<(u64, usize, usize)>> = BinaryHeap::new();

    for i in 0..jbox.len() - 1 {
        for j in i + 1..jbox.len() {
            let distance = jbox[i].distance_squared(&jbox[j]);
            heap.push(Reverse((distance, i, j)));
        }
    }

    let mut connections = 0;

    let mut circuits: Vec<HashSet<usize>> = Vec::new();

    while connections < nb_conn {
        if let Some(Reverse((_d, i, j))) = heap.pop() {
            let is: Vec<usize> = circuits
                .iter()
                .enumerate()
                .filter_map(|(idx, v)| if v.contains(&i) { Some(idx) } else { None })
                .collect();
            let js: Vec<usize> = circuits
                .iter()
                .enumerate()
                .filter_map(|(idx, v)| if v.contains(&j) { Some(idx) } else { None })
                .collect();

            if is.len() > 1 || js.len() > 1 {
                println!("Something didn't work");
            } else if is.len() == 1 && js.len() == 1 {
                let idx_i = is[0];
                let idx_j = js[0];
                if idx_i == idx_j {
                    //nothing to do, the connection is already there
                    connections += 1;
                    continue;
                } else {
                    //connection between two circuits, we have to merge them
                    
                    let bs = circuits[idx_j].clone();

                    for b in bs {
                        circuits[idx_i].insert(b);
                    }
                    circuits.remove(idx_j);
                    connections += 1;
                }
            } else if is.len() == 1 {
                let idx_i = is[0];
                circuits[idx_i].insert(j);
                connections += 1;
            } else if js.len() == 1 {
                let idx_j = js[0];
                circuits[idx_j].insert(i);
                connections += 1;
            } else {
                //neither index is found, new circuit
                let new_circuit = HashSet::from([i, j]);
                circuits.push(new_circuit);
                connections += 1;
            }
        }
    }

    circuits.sort_by_key(|v| Reverse(v.len()));

    let mut res: u64 = 1;
    for i in 0..3 {
        res *= circuits[i].len() as u64;
    }

    res
}

fn connect_all(jbox: &Vec<JBox>) -> u64 {
    let mut heap: BinaryHeap<Reverse<(u64, usize, usize)>> = BinaryHeap::new();

    let cable;

    for i in 0..jbox.len() - 1 {
        for j in i + 1..jbox.len() {
            let distance = jbox[i].distance_squared(&jbox[j]);
            heap.push(Reverse((distance, i, j)));
        }
    }


    let mut circuits: Vec<HashSet<usize>> = (0..jbox.len()).map(|x| HashSet::from([x])).collect();

    loop {
        if let Some(Reverse((_d, i, j))) = heap.pop() {
            let is: Vec<usize> = circuits
                .iter()
                .enumerate()
                .filter_map(|(idx, v)| if v.contains(&i) { Some(idx) } else { None })
                .collect();
            let js: Vec<usize> = circuits
                .iter()
                .enumerate()
                .filter_map(|(idx, v)| if v.contains(&j) { Some(idx) } else { None })
                .collect();

            if is.len() > 1 || js.len() > 1 {
                println!("Something didn't work");
            } else if is.len() == 1 && js.len() == 1 {
                let idx_i = is[0];
                let idx_j = js[0];
                if idx_i == idx_j {
                    //nothing to do, the connection is already there
                    continue;
                } else {
                    //connection between two circuits, we have to merge them
                    
                    let bs = circuits[idx_j].clone();

                    for b in bs {
                        circuits[idx_i].insert(b);
                    }
                    circuits.remove(idx_j);

                    //check if we reached the goal
                    if circuits.len() == 1 {
                        cable = jbox[i].x * jbox[j].x;
                        break;
                    }
                }
            } else if is.len() == 1 {
                let idx_i = is[0];
                circuits[idx_i].insert(j);
            } else if js.len() == 1 {
                let idx_j = js[0];
                circuits[idx_j].insert(i);
            } else {
                //neither index is found, new circuit
                let new_circuit = HashSet::from([i, j]);
                circuits.push(new_circuit);
            }
        }
    }

    circuits.sort_by_key(|v| Reverse(v.len()));

    cable
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let start = Instant::now();

    let mut junction_boxes = Vec::new();

    for line in fs::read_to_string(file_path)?.lines() {
        let mut jb = line.split(',');
        let b = JBox {
            x: jb.next().unwrap().parse::<u64>()?,
            y: jb.next().unwrap().parse::<u64>()?,
            z: jb.next().unwrap().parse::<u64>()?,
        };
        junction_boxes.push(b);
    }

    //println!("{:?}", junction_boxes);

    let duration = start.elapsed();

    println!(
        "Parse: {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();

    let circuits = connect_boxes(&junction_boxes, 10);

    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        circuits,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();

    let circuits = connect_all(&junction_boxes);

    let duration = start.elapsed();
    println!(
        "Part2: {} | {}s {}ms {}µs {}ns",
        circuits,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
