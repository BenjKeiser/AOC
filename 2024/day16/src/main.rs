use grid::{Direction, Grid, Point};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
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
    dir: Direction,
    path: Vec<Point>,
}

// Custom ordering for the priority queue (min-heap based on cost)
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by cost (reverse for min-heap behavior)
        match other.cost.cmp(&self.cost) {
            Ordering::Equal => {
                // Tie-breaker: compare by pos (Point)
                match self.pos.cmp(&other.pos) {
                    Ordering::Equal => self.dir.cmp(&other.dir), // Tie-breaker: compare by dir
                    other_order => other_order,
                }
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
fn solve_maze(maze: &Grid, start: &Point, start_dir: &Direction, end: &Point) -> (usize, usize) {
    let mut visited: Vec<Vec<Vec<usize>>> = vec![vec![vec![MAX; 4]; maze[0].len()]; maze.len()];

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();

    let mut best_path_cost = MAX;
    let mut best_path: Vec<Point>;
    let mut best_path_collection: HashMap<usize, HashSet<Point>> = HashMap::new();
    let mut unique_tiles = 0;

    //push start element
    heap.push(Node {
        cost: 0,
        pos: *start,
        dir: *start_dir,
        path: Vec::new(),
    });
    visited[start.y][start.x][start_dir.to_idx().unwrap()] = 0;
    let mut turn = start_dir.turn_left().unwrap();
    heap.push(Node {
        cost: 1000,
        pos: *start,
        dir: turn,
        path: Vec::new(),
    });
    visited[start.y][start.x][turn.to_idx().unwrap()] = 1000;
    turn = turn.turn_left().unwrap();
    heap.push(Node {
        cost: 2000,
        pos: *start,
        dir: turn,
        path: Vec::new(),
    });
    visited[start.y][start.x][turn.to_idx().unwrap()] = 2000;
    turn = start_dir.turn_right().unwrap();
    heap.push(Node {
        cost: 1000,
        pos: *start,
        dir: turn,
        path: Vec::new(),
    });
    visited[start.y][start.x][turn.to_idx().unwrap()] = 1000;

    while let Some(mut node) = heap.pop() {
        //Skip nodes that have been processed with shorter paths
        if node.cost > visited[node.pos.y][node.pos.x][node.dir.to_idx().unwrap()] {
            continue;
        }
        visited[node.pos.y][node.pos.x][node.dir.to_idx().unwrap()] = node.cost;

        node.path.push(node.pos);

        //add the turns
        heap.push(Node {
            cost: node.cost + 1000,
            pos: node.pos,
            dir: node.dir.turn_left().unwrap(),
            path: node.path.clone(),
        });
        heap.push(Node {
            cost: node.cost + 1000,
            pos: node.pos,
            dir: node.dir.turn_right().unwrap(),
            path: node.path.clone(),
        });

        if maze.is_move_valid(&node.pos, &node.dir) {
            if let Some(next) = node.pos + node.dir {
                if next == *end {
                    if node.cost + 1 < best_path_cost {
                        best_path_cost = node.cost + 1;
                        best_path = node.path;
                        best_path.push(next);
                        let mut hs: HashSet<Point> = HashSet::new();
                        for e in &best_path {
                            hs.insert(*e);
                        }
                        best_path_collection.insert(node.cost+1, hs);
                    }
                    else if node.cost+1 == best_path_cost {
                        if let Some(hs) = best_path_collection.get_mut(&best_path_cost){
                            for e in &node.path {
                                hs.insert(*e);
                            }
                        }
                    }
                } else if maze[next.y][next.x] != '#' {
                    heap.push(Node {
                        cost: node.cost + 1,
                        pos: next,
                        dir: node.dir,
                        path: node.path.clone(),
                    });
                }
            }
        }
    }

    if let Some(tiles) = best_path_collection.get(&best_path_cost){
        unique_tiles = tiles.len();
    }
    (best_path_cost, unique_tiles)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut maze: Grid = Grid::new();

    let mut y: usize = 0;

    let mut start_p: Point = Point { y: 0, x: 0 };
    let mut end_p: Point = Point { y: 0, x: 0 };

    for line in fs::read_to_string(file_path)?.lines() {
        if line.len() > 0 {
            let cs: Vec<_> = line.chars().collect();

            if let Some(x) = cs.iter().position(|&c| c == 'S') {
                start_p = Point { y: y, x: x };
            }

            if let Some(x) = cs.iter().position(|&c| c == 'E') {
                end_p = Point { y: y, x: x };
            }

            maze.push(cs);
        }
        y += 1;
    }

    println!("{}", maze);

    let start = Instant::now();
    let (score, tiles) = solve_maze(&maze, &start_p, &Direction { x: 1, y: 0 }, &end_p);
    let duration = start.elapsed();
    println!("Score: {}, Tiles: {tiles}| {}s", score, duration.as_secs_f32());

    //let start = Instant::now();
    //
    //let duration = start.elapsed();
    //println!("Part2: {} | {}s", 2, duration.as_secs_f32());

    Ok(())
}
