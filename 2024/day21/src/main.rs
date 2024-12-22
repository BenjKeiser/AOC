use grid::{Direction, Grid, Point};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::time::Instant;
use std::usize::MAX;

static DIRECTIONS: &'static [char] = &['^', '>', 'v', '<'];

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Node {
    cost: usize,
    pos: Point,
    dirs: Vec<char>,
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

fn get_cost(d1: &char, d2: &char) -> usize {
    let p1 = get_dir_pos(d1);
    let p2 = get_dir_pos(d2);
    ((p1.x as i32 - p2.x as i32).abs() + (p2.y as i32 - p2.x as i32).abs()) as usize
}

//use dijkstra
fn dijkstra(pad: &Grid, start: &Point, end: &Point) -> Vec<char> {
    let mut visited: Vec<Vec<usize>> = vec![vec![MAX; pad[0].len()]; pad.len()];

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();

    let mut best_path_cost = MAX;

    let mut shortest_dirs: Vec<char> = Vec::new();

    //push start element
    heap.push(Node {
        cost: 0,
        pos: *start,
        dirs: Vec::new(),
    });

    while let Some(node) = heap.pop() {
        //Skip nodes that have been processed with shorter paths
        if node.cost > visited[node.pos.y][node.pos.x] {
            continue;
        }
        visited[node.pos.y][node.pos.x] = node.cost;

        if node.pos == *end {
            if node.dirs.len() < best_path_cost {
                best_path_cost = node.dirs.len();
                shortest_dirs = node.dirs.clone();
                shortest_dirs.push('A');
            }
        }

        for d in DIRECTIONS {
            let dir = Direction::arrow_char_to_dir(d).unwrap();
            if pad.is_move_valid(&node.pos, &dir) {
                if let Some(next) = node.pos + dir {
                    if pad[next.y][next.x] != ' ' {
                        let mut dirs = node.dirs.clone();
                        dirs.push(*d);
                        if let Some(old_dir) = node.dirs.last() {
                            let mc = get_cost(old_dir, d);
                            heap.push(Node {
                                cost: node.cost + mc,
                                pos: next,
                                dirs: dirs,
                            })
                        } else {
                            // no direction, we start from 'A'
                            let mc = get_cost(&'A', d);
                            heap.push(Node {
                                cost: node.cost + mc,
                                pos: next,
                                dirs: dirs,
                            })
                        }
                    }
                }
            }
        }
    }

    shortest_dirs
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn get_num_pos(button: &char) -> Point {
    match *button {
        '7' => Point { x: 0, y: 0 },
        '8' => Point { x: 1, y: 0 },
        '9' => Point { x: 2, y: 0 },
        '4' => Point { x: 0, y: 1 },
        '5' => Point { x: 1, y: 1 },
        '6' => Point { x: 2, y: 1 },
        '1' => Point { x: 0, y: 2 },
        '2' => Point { x: 1, y: 2 },
        '3' => Point { x: 2, y: 2 },
        '0' => Point { x: 1, y: 3 },
        'A' => Point { x: 2, y: 3 },
        _ => Point { x: 0, y: 3 },
    }
}

fn get_dir_pos(button: &char) -> Point {
    match *button {
        '^' => Point { x: 1, y: 0 },
        'A' => Point { x: 2, y: 0 },
        '<' => Point { x: 0, y: 1 },
        'v' => Point { x: 1, y: 1 },
        '>' => Point { x: 2, y: 1 },
        _ => Point { x: 0, y: 0 },
    }
}

fn get_num_kp() -> Grid {
    let mut kp_num = Grid::with_size(4, 3, ' ');

    kp_num[0][0] = '7';
    kp_num[0][1] = '8';
    kp_num[0][2] = '9';
    kp_num[1][0] = '4';
    kp_num[1][1] = '5';
    kp_num[1][2] = '6';
    kp_num[2][0] = '1';
    kp_num[2][1] = '2';
    kp_num[2][2] = '3';
    kp_num[3][1] = '0';
    kp_num[3][2] = 'A';

    kp_num
}

fn get_dir_kp() -> Grid {
    let mut kp_dir = Grid::with_size(2, 3, ' ');

    kp_dir[0][1] = '^';
    kp_dir[0][2] = 'A';
    kp_dir[1][0] = '<';
    kp_dir[1][1] = 'v';
    kp_dir[1][2] = '>';

    kp_dir
}

fn push_button(num: &Grid, dir: &Grid, start: char, button: char) -> usize {
    //Note, the robots always end on A as they need that button to execut the button
    let mut buttons = Vec::new();

    println!("{start} => {button}");

    let start_pos = get_num_pos(&start);
    let end_pos = get_num_pos(&button);

    let a_pos_robot = Point { x: 2, y: 0 };

    //moves the first robot has to perform to do the num pad push for the button
    let mut num_dir = dijkstra(num, &start_pos, &end_pos);

    println!("{:?}", num_dir);

    //moves the second robot has to perform for the first robot to push the buttons
    let mut s = a_pos_robot;
    let mut dir_dir: Vec<char> = Vec::new();
    for e in num_dir {
        let end = get_dir_pos(&e);
        let mut r_dirs = dijkstra(dir, &s, &end);
        s = end;
        dir_dir.append(&mut r_dirs);
    }

    println!("{:?}", dir_dir);

    //moves I have to perform for the second robot to push the buttons
    let mut s = a_pos_robot;
    for e in dir_dir {
        let end = get_dir_pos(&e);
        let mut r_dirs = dijkstra(dir, &s, &end);
        s = end;
        buttons.append(&mut r_dirs);
    }

    println!("{:?}", buttons);

    buttons.len()
}

fn get_complexity(codes: &Vec<(usize, Vec<char>)>) -> usize {
    let mut complexity = 0;
    let kp_num = get_num_kp();
    let kp_dir = get_dir_kp();

    for (comp, code) in codes {
        let mut keypresses = 0;
        let mut start = 'A';
        for c in code {
            keypresses += push_button(&kp_num, &kp_dir, start, *c);
            start = *c
        }
        println!("{:?} => {} * {}", code, keypresses, comp);
        complexity += comp * keypresses;
    }

    complexity
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let mut codes: Vec<(usize, Vec<char>)> = Vec::new();

    for line in fs::read_to_string(file_path)?.lines() {
        codes.push((line[..3].parse::<usize>()?, line.chars().collect()));
    }

    let start = Instant::now();
    let complexity = get_complexity(&codes);
    let duration = start.elapsed();
    println!(
        "Part1: {complexity} | {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();
    let bananas = 0;
    let duration = start.elapsed();
    println!(
        "Part2: {bananas} | {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
