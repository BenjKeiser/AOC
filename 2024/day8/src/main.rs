use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn calc_pos(
    a: (usize, usize),
    b: (usize, usize),
    y_max: usize,
    x_max: usize,
) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::new();

    let y_diff: i32 = a.0 as i32 - b.0 as i32;
    let x_diff: i32 = a.1 as i32 - b.1 as i32;

    //first position
    let p_y = a.0 as i32 + y_diff;
    let p_x = a.1 as i32 + x_diff;

    if p_y >= 0 && p_y <= y_max as i32 && p_x >= 0 && p_x <= x_max as i32 {
        positions.push((p_y as usize, p_x as usize));
    }

    //second position
    let p_y = b.0 as i32 - y_diff;
    let p_x = b.1 as i32 - x_diff;

    if p_y >= 0 && p_y <= y_max as i32 && p_x >= 0 && p_x <= x_max as i32 {
        positions.push((p_y as usize, p_x as usize));
    }

    positions
}

fn get_antinodes(antennas: &HashMap<char, Vec<(usize, usize)>>, y_max: usize, x_max: usize) -> u64 {
    let mut nb_nodes = 0;
    let mut a_nodes: Vec<(usize, usize)> = Vec::new();
    //println!("Antennas: {:?}", antennas);

    for key in antennas.keys() {
        if let Some(a_list) = antennas.get(key) {
            for i in 0..a_list.len() {
                for z in i + 1..a_list.len() {
                    let pos = calc_pos(a_list[i], a_list[z], y_max, x_max);
                    for p in pos {
                        if !a_nodes.contains(&p) {
                            a_nodes.push(p);
                            nb_nodes += 1;
                            //println!("Added Antinode: {:?},{:?} -> {:?}", a_list[i], a_list[z], p);
                        }
                    }
                }
            }
        }
    }

    nb_nodes
}

fn calc_pos_resonance(
    a: (usize, usize),
    b: (usize, usize),
    y_max: usize,
    x_max: usize,
) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::new();

    let y_diff: i32 = a.0 as i32 - b.0 as i32;
    let x_diff: i32 = a.1 as i32 - b.1 as i32;

    //first position
    let mut p_y = a.0 as i32;
    let mut p_x = a.1 as i32;

    loop {
        p_y = p_y + y_diff;
        p_x = p_x + x_diff;

        if p_y >= 0 && p_y <= y_max as i32 && p_x >= 0 && p_x <= x_max as i32 {
            positions.push((p_y as usize, p_x as usize));
        } else {
            break;
        }
    }

    //second position
    p_y = b.0 as i32;
    p_x = b.1 as i32;

    loop {
        p_y = p_y - y_diff;
        p_x = p_x - x_diff;

        if p_y >= 0 && p_y <= y_max as i32 && p_x >= 0 && p_x <= x_max as i32 {
            positions.push((p_y as usize, p_x as usize));
        } else {
            break;
        }
    }

    positions
}

fn get_antinodes_resonance(
    antennas: &HashMap<char, Vec<(usize, usize)>>,
    y_max: usize,
    x_max: usize,
) -> u64 {
    let mut nb_nodes = 0;
    let mut a_nodes: Vec<(usize, usize)> = Vec::new();
    //println!("Antennas: {:?}", antennas);

    for key in antennas.keys() {
        if let Some(a_list) = antennas.get(key) {
            for i in 0..a_list.len() {
                for z in i+1..a_list.len() {
                    let pos = calc_pos_resonance(a_list[i], a_list[z], y_max, x_max);
                    for p in pos {
                        if !a_nodes.contains(&p) {
                            a_nodes.push(p);
                            nb_nodes += 1;
                            //println!("Added Antinode: {:?},{:?} -> {:?}", a_list[i], a_list[z], p);
                        }
                    }
                }
            }
            //each antenna position can be an antinode
            for p in a_list{
                if !a_nodes.contains(p) {
                    a_nodes.push(*p);
                    nb_nodes += 1;
                    //println!("Added Antinode: {:?}", p);
                }
            }
        }
    }

    nb_nodes
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    let mut y: usize = 0;
    for line in fs::read_to_string(file_path)?.lines() {
        let mut x: usize = 0;
        map.push(line.chars().collect());
        for c in line.chars() {
            if c.is_alphanumeric() {
                match antennas.get_mut(&c) {
                    Some(ants) => ants.push((y, x)),
                    None => {
                        let a_pos: Vec<(usize, usize)> = vec![(y, x)];
                        antennas.insert(c, a_pos);
                    }
                }
            }
            x += 1;
        }
        y += 1;
    }

    let a_nodes = get_antinodes(&antennas, map.len() - 1, map[0].len() - 1);
    println!("part1: {a_nodes}");
    let a_nodes = get_antinodes_resonance(&antennas, map.len() - 1, map[0].len() - 1);
    println!("part2: {a_nodes}");
    Ok(())
}
//1711 for part2
