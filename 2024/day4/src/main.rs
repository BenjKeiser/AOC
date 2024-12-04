use std::fs;
use std::fmt;
use std::error::Error;
use std::ffi::OsString;
use std::env;
use std::collections::VecDeque;
use std::slice::Iter;

pub struct Direction {
    x: i32,
    y: i32,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [Direction { x: -1, y: -1}, 
                                             Direction { x: 0, y: -1}, 
                                             Direction { x: 1, y: -1}, 
                                             Direction { x: 1, y: 0}, 
                                             Direction { x: 1, y: 1}, 
                                             Direction { x: 0, y: 1}, 
                                             Direction { x: -1, y: 1}, 
                                             Direction { x: -1, y: 0}];
        DIRECTIONS.iter()
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn find_next_char(c_list: &VecDeque<char>, map: &Vec<Vec<char>>, x: usize, y: usize, dir: &Direction) -> bool {
    let mut ret = false;
    if c_list.len() != 0 {
        let c = c_list[0];
        let mut next = c_list.clone();
        next.pop_front();

        let x_coord = (x as i32) + dir.x;
        let y_coord = (y as i32) + dir.y;

        if x_coord >= 0 && y_coord >= 0 {
            let x_coord = x_coord as usize;
            let y_coord = y_coord as usize;
            if x_coord < map[0].len() && y_coord < map.len() {
                if map[y_coord][x_coord] == c {
                    ret = find_next_char(&next, map, x_coord, y_coord, dir);
                }
            }
        }  
    }
    else {
        ret = true;
    }

    ret
}

fn find_words(word: &str, map: &Vec<Vec<char>>) -> i32 {
    let mut w_vec: VecDeque<char> = word.chars().collect();
    let mut sum = 0;
    let c = w_vec[0];
    
    w_vec.pop_front();
    //find_next_char(&w_vec, map, 5, 0, &Direction {x: 1, y: 0});
    for i in 0..map.len() {
        for pos in map[i].iter().enumerate().filter(|(_, x)| **x == c).map(|(idx, _)| idx) {
            for dir in Direction::iterator() {
                if find_next_char(&w_vec, map, pos, i, dir) {
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn find_x_mas(map: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    //find_next_char(&w_vec, map, 5, 0, &Direction {x: 1, y: 0});
    for y in 1..map.len()-1 {
        for x in 1..map[y].len()-1 {
            if map[y][x] == 'A' {
                if (map[y-1][x-1] == 'S' && map[y+1][x+1] == 'M') ||
                   (map[y-1][x-1] == 'M' && map[y+1][x+1] == 'S') {
                    if (map[y-1][x+1] == 'S' && map[y+1][x-1] == 'M') ||
                       (map[y-1][x+1] == 'M' && map[y+1][x-1] == 'S') {
                        sum += 1;
                   }
                }
            }
        }
    }
    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let mut word_map: Vec<Vec<char>> = Vec::new();
    for line in fs::read_to_string(file_path)?.lines(){
        let char_vec: Vec<char> = line.chars().collect();
        word_map.push(char_vec);
    }
    let words = find_words("XMAS", &word_map);
    println!("part1: {words}");
    let x_mas = find_x_mas(&word_map);
    println!("part2: {x_mas}");
    Ok(())
}
