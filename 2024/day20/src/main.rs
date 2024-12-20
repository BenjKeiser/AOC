use grid::{Grid, Point};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
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
fn get_seconds(
    memory: &Grid,
    start: &Point,
    end: &Point,
    cheat: &Point,
    to_beat: usize,
) -> Option<usize> {
    let mut visited: Vec<Vec<usize>> = vec![vec![MAX; memory[0].len()]; memory.len()];

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();

    let mut best_path_cost = MAX;

    //push start element
    heap.push(Node {
        cost: 0,
        pos: *start,
    });

    while let Some(node) = heap.pop() {
        //Skip nodes that have been processed with shorter paths
        if node.cost >= visited[node.pos.y][node.pos.x] {
            continue;
        }
        visited[node.pos.y][node.pos.x] = node.cost;

        //we did not manage to beat the clock -> return early
        if node.cost > to_beat {
            return None;
        }

        if node.pos == *end {
            if node.cost < best_path_cost {
                best_path_cost = node.cost;
            }
        }

        for next in memory.get_neighbours(&node.pos, false) {
            if memory[next.y][next.x] != '#' || next == *cheat {
                heap.push(Node {
                    cost: node.cost + 1,
                    pos: next,
                });
            }
        }
    }

    Some(best_path_cost)
}

fn get_cheats(track: &Grid) -> Vec<Point> {
    let mut cheats: Vec<Point> = Vec::new();
    for (y, row) in track.iter().enumerate() {
        for (x, b) in row.iter().enumerate() {
            if !track.is_boundary(&Point { y: y, x: x }) {
                if *b == '#' {
                    let neighbours = track.get_neighbours(&Point { y: y, x: x }, false);
                    if neighbours
                        .iter()
                        .filter(|&p| track[p.y][p.x] != '#')
                        .count()
                        >= 2
                    {
                        cheats.push(Point { x: x, y: y });
                    }
                }
            }
        }
    }
    cheats
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut track: Grid = Grid::new();

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

            track.push(cs);
        }
        y += 1;
    }

    let start = Instant::now();
    let all_cheats = get_cheats(&track);
    let mut cheats= 0;

    if let Some(time_to_beat) = get_seconds(&track, &start_p, &end_p, &Point { x: 0, y: 0 }, MAX) {
        println!("Time To Beat: {time_to_beat}");
        cheats = 
            all_cheats
                .iter()
                .filter(|&c| {
                    if let Some(time) = get_seconds(&track, &start_p, &end_p, c, time_to_beat-99) {
                        100 <= (time_to_beat - time)
                    } else {
                        false
                    }
                })
                .count();
    }
    let duration = start.elapsed();
    println!(
        "Part1: {cheats} | {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();
    let dunno = 0;
    let duration = start.elapsed();
    println!(
        "Part2: {dunno} | {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );
    Ok(())
}
