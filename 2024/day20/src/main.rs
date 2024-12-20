use grid::{Grid, Point};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
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
fn dijkstra(track: &Grid, start: &Point, end: &Point) -> usize {
    let mut visited: Vec<Vec<usize>> = vec![vec![MAX; track[0].len()]; track.len()];

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

        if node.pos == *end {
            if node.cost < best_path_cost {
                best_path_cost = node.cost;
            }
        }

        for next in track.get_neighbours(&node.pos, false) {
            if track[next.y][next.x] != '#' || next == *end {
                heap.push(Node {
                    cost: node.cost + 1,
                    pos: next,
                });
            }
        }
    }

    best_path_cost
}

//use dijkstra
fn get_nb_better_cheats(
    track: &Grid,
    start: &Point,
    end: &Point,
    cheats: &Vec<(Point, Vec<(Point, usize)>)>,
    to_beat: usize,
) -> usize {
    let mut better_cheats = 0;

    let mut cost_map_bc: HashMap<Point, usize> = HashMap::new();
    let mut cost_map_ac: HashMap<Point, usize> = HashMap::new();

    for (c, exits) in cheats {
        let start_cost;
        if let Some(val) = cost_map_bc.get(c) {
            start_cost = *val;
        } else {
            start_cost = dijkstra(track, start, c);
            cost_map_bc.insert(*c, start_cost);
        }
        for (e, cheat_cost) in exits {
            let end_cost;
            if let Some(val) = cost_map_ac.get(e) {
                end_cost = *val;
            } else {
                end_cost = dijkstra(track, e, end);
                cost_map_ac.insert(*e, end_cost);
            }
            if (start_cost + cheat_cost + end_cost) <= to_beat {
                /*println!(
                    "{} {} => {} {} {} = {}",
                    c,
                    e,
                    start_cost,
                    cheat_cost,
                    end_cost,
                    (start_cost + cheat_cost + end_cost)
                ); */
                better_cheats += 1;
            }
        }
    }

    better_cheats
}

fn get_cheats(track: &Grid, duration: usize) -> Vec<(Point, Vec<(Point, usize)>)> {
    let mut cheats: Vec<_> = Vec::new();
    for (y, row) in track.iter().enumerate() {
        for (x, b) in row.iter().enumerate() {
            if *b != '#' {
                let coord = Point { y: y, x: x };
                let reachable = track
                    .get_reachable(&coord, duration)
                    .iter()
                    .filter(|&(p, _)| track[p.y][p.x] != '#')
                    .map(|&x| x)
                    .collect::<Vec<(Point, usize)>>();
                cheats.push((coord, reachable));
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

    let all_cheats = get_cheats(&track, 2);
    let time_to_beat = dijkstra(&track, &start_p, &end_p);
    let cheats = get_nb_better_cheats(&track, &start_p, &end_p, &all_cheats, time_to_beat - 100);

    let duration = start.elapsed();
    println!(
        "Part1: {cheats} | {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();
    let all_cheats = get_cheats(&track, 20);
    let cheats = get_nb_better_cheats(&track, &start_p, &end_p, &all_cheats, time_to_beat - 100);
    let duration = start.elapsed();
    println!(
        "Part2: {cheats} | {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
