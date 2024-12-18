use grid::{Grid, Point};
use std::cmp::Ordering;
use std::collections::{BinaryHeap};
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::time::Instant;
use std::usize::MAX;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Node {
    cost: usize,
    pos: Point,
    path: Vec<Point>,
}

// Custom ordering for the priority queue (min-heap based on cost)
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by cost (reverse for min-heap behavior)
        match other.cost.cmp(&self.cost) {
            Ordering::Equal => {
                // Tie-breaker: compare by pos (Point)
                self.pos.cmp(&other.pos)
            }
            other_order => other_order,
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

//use dijkstra
fn get_steps(memory: &Grid, start: &Point, end: &Point) -> usize {
    let mut visited: Vec<Vec<usize>> = vec![vec![MAX; memory[0].len()]; memory.len()];

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();

    let mut best_path_cost = MAX;
    let mut best_path: Vec<Point> = Vec::new();

    //push start element
    heap.push(Node {
        cost: 0,
        pos: *start,
        path: Vec::new(),
    });
    visited[start.y][start.x] = 0;

    while let Some(mut node) = heap.pop() {
        //Skip nodes that have been processed with shorter paths
        if node.cost > visited[node.pos.y][node.pos.x] {
            continue;
        }
        visited[node.pos.y][node.pos.x] = node.cost;

        node.path.push(node.pos);

        if node.pos == *end {
            if node.cost < best_path_cost {
                best_path_cost = node.cost;
                best_path = node.path.clone();
            }
        }

        for next in memory.get_neighbours(&node.pos, false) {
            if memory[next.y][next.x] != '#' {
                heap.push(Node {
                    cost: node.cost + 1,
                    pos: next,
                    path: node.path.clone(),
                });
            }
        }
    }

    let mut grid = memory.clone();

    for i in &best_path {
        grid[i.y][i.x] = 'O';
    }
    println!("{grid}");

    best_path_cost
}

fn drop_ram(mem: &Grid, ram: &Vec<Point>, amount: usize) -> Grid {
    let mut memory = mem.clone();
    for i in 0..amount {
        memory[ram[i].y][ram[i].x] = '#';
    }
    memory
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut memory: Grid = Grid::with_size(71, 71, '.');

    let start_p: Point = Point { y: 0, x: 0 };
    let end_p: Point = Point {
        y: memory.len() - 1,
        x: memory[0].len() - 1,
    };

    let mut ram: Vec<Point> = Vec::new();

    for line in fs::read_to_string(file_path)?.lines() {
        let str: Vec<&str> = line.splitn(2, ',').collect();
        ram.push(Point {
            x: str[0].parse::<usize>()?,
            y: str[1].parse::<usize>()?,
        });
    }

    let start = Instant::now();
    memory = drop_ram(&memory, &ram, 1024);
    let steps = get_steps(&memory, &start_p, &end_p);
    let duration = start.elapsed();
    println!(
        "Part1: {steps} | {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    //let start = Instant::now();
    //
    //let duration = start.elapsed();
    //println!("Part2: {} | {}s", 2, duration.as_secs_f32());

    Ok(())
}
