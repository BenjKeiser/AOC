use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs;

#[derive(Clone, PartialEq, Debug)]
pub struct Plots {
    plot_name: char,
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

fn get_plots(garden: &Vec<Vec<char>>) -> Vec<Plots> {
    let mut plots: Vec<Plots> = Vec::new();
    let mut visited_plots: HashSet<(usize, usize)> = HashSet::new();

    //put a starting point into a vector
    //explore the plot of the starting points -> use a different vector as the exploration queue
    //if the points are of the same plot, mark them as visited in the HashSet
    //put neighbours with a different plot on the first vector
    //loop until both vectors are empty

    plots
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut garden: Vec<Vec<char>> = Vec::new();

    for line in fs::read_to_string(file_path)?.lines() {
        garden.push(line.chars().collect());
    }

    Ok(())
}
