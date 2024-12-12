use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::time::Instant;

#[derive(Clone, PartialEq, Debug)]
pub struct Plots {
    id: char,
    plots: Vec<(usize, usize)>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Direction {
    x: i32,
    y: i32,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

static DIRECTIONS: &'static [Direction] = &[
    Direction { x: 0, y: -1 },
    Direction { x: 1, y: 0 },
    Direction { x: 0, y: 1 },
    Direction { x: -1, y: 0 },
];

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn corner_count(id: char, coords: &(usize, usize), garden: &Vec<Vec<char>>) -> u64 {
    let mut corners = 0;


    corners
}

fn boundary_count(id: char, coords: &(usize, usize), garden: &Vec<Vec<char>>) -> u64 {
    let mut boundaries = 0;

    let next = get_neighbours(garden.len() as i32, garden[0].len() as i32, coords);

    //get boundaries which are off map
    boundaries += 4 - next.len() as u64;

    for n in next {
        if garden[n.0][n.1] != id {
            boundaries += 1;
        }
    }

    boundaries
}

fn get_neighbours(rows: i32, cols: i32, coords: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::new();

    for d in DIRECTIONS {
        let y = coords.0 as i32 - d.y;
        let x = coords.1 as i32 - d.x;
        if y >= 0 && y <= rows - 1 && x >= 0 && x <= cols - 1 {
            neighbours.push((y as usize, x as usize));
        }
    }

    neighbours
}

fn get_score_p1(garden: &Vec<Vec<char>>, plots: &Vec<Plots>) -> u64 {
    let mut score = 0;
    for p in plots {
        let area = p.plots.len() as u64;
        let mut boundary = 0;
        for i in 0..p.plots.len() {
            boundary += boundary_count(p.id, &p.plots[i], garden);
        }
        score += area * boundary;
    }

    score
}

fn get_score_p2(garden: &Vec<Vec<char>>, plots: &Vec<Plots>) -> u64 {
    let mut score = 0;
    for p in plots {
        let area = p.plots.len() as u64;
        let mut corners = 0;
        for i in 0..p.plots.len() {
            corners += corner_count(p.id, &p.plots[i], garden);
        }
        score += area * corners;
    }

    score
}

fn get_plots(garden: &Vec<Vec<char>>) -> Vec<Plots> {
    let mut plots: Vec<Plots> = Vec::new();
    let mut visited_plots: HashSet<(usize, usize)> = HashSet::new();

    let mut plot_queue: Vec<(usize, usize)> = Vec::new();
    let mut current_plot_queue: Vec<(usize, usize)> = Vec::new();

    plot_queue.push((0, 0));

    while !plot_queue.is_empty() {
        if let Some((y, x)) = plot_queue.pop() {
            let current_plot_id = garden[y][x];
            let mut current_plot: Vec<(usize, usize)> = Vec::new();
            current_plot_queue.push((y, x));
            while !current_plot_queue.is_empty() {
                if let Some((y_c, x_c)) = current_plot_queue.pop() {
                    if garden[y_c][x_c] != current_plot_id {
                        //wrong plot skip
                        continue;
                    }

                    if visited_plots.contains(&(y_c, x_c)) {
                        //already were here skip
                        continue;
                    }
                    visited_plots.insert((y_c, x_c));

                    //we add the coordinates to the combined plot
                    current_plot.push((y_c, x_c));

                    let next =
                        get_neighbours(garden.len() as i32, garden[0].len() as i32, &(y_c, x_c));
                    for n in next {
                        //println!("({y_c}, {x_c}) [{current_plot_id}]-> {:?} [{}]", n, garden[n.0][n.1]);
                        if garden[n.0][n.1] == current_plot_id {
                            current_plot_queue.push((n.0, n.1));
                        } else {
                            plot_queue.push((n.0, n.1));
                        }
                    }
                }
            }
            //no more neighbours in this plot -> we put it on the list if it is valid
            if !current_plot.is_empty() {
                plots.push(Plots {
                    id: current_plot_id,
                    plots: current_plot,
                });
                //println!("Added")
            }
        }
    }

    plots
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut garden: Vec<Vec<char>> = Vec::new();

    for line in fs::read_to_string(file_path)?.lines() {
        garden.push(line.chars().collect());
    }

    let start = Instant::now();
    let plots = get_plots(&garden);
    let score = get_score_p1(&garden, &plots);
    let duration = start.elapsed();
    println!("Part1: {score} | {}s", duration.as_secs_f32());

    let start = Instant::now();
    let plots = get_plots(&garden);
    let score = get_score_p2(&garden, &plots);
    let duration = start.elapsed();
    println!("Part2: {score} | {}s", duration.as_secs_f32());
    //println!("{:?}", plots);
    Ok(())
}
