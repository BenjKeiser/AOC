use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Mul, Sub};
use std::ops::{Deref, DerefMut};

#[derive(Clone, PartialEq, Debug)]
pub struct Grid(Vec<Vec<char>>);

static DIRECTIONS: &'static [Direction] = &[
    Direction { x: 0, y: -1 },
    Direction { x: 1, y: 0 },
    Direction { x: 0, y: 1 },
    Direction { x: -1, y: 0 },
    Direction { x: -1, y: -1 },
    Direction { x: 1, y: -1 },
    Direction { x: -1, y: 1 },
    Direction { x: 1, y: 1 },
];

impl Grid {
    /// Creates a new, empty Grid
    pub fn new() -> Self {
        Grid(Vec::new())
    }

    /// Creates a Grid with a predefined size and fills it with a default value
    pub fn with_size(rows: usize, cols: usize, default: char) -> Self {
        let grid = vec![vec![default; cols]; rows];
        Grid(grid)
    }

    pub fn is_move_valid(&self, pos: &Point, dir: &Direction) -> bool {
        match dir {
            Direction { x: -1, y: 0 } => pos.x > 0,
            Direction { x: 0, y: -1 } => pos.y > 0,
            Direction { x: 1, y: 0 } => pos.x < self[0].len() - 1,
            Direction { x: 0, y: 1 } => pos.y < self.len() - 1,
            Direction { x: -1, y: -1 } => pos.x > 0 && pos.y > 0,
            Direction { x: 1, y: -1 } => pos.x < self[0].len() - 1 && pos.y > 0,
            Direction { x: -1, y: 1 } => pos.x > 0 && pos.y < self.len() - 1,
            Direction { x: 1, y: 1 } => pos.x < self[0].len() - 1 && pos.y < self.len() - 1,
            _ => false,
        }
    }

    pub fn get_neighbours(&self, pos: &Point, diag: bool) -> Vec<Point> {
        let mut neighbours: Vec<Point> = Vec::new();
        for dir in DIRECTIONS {
            if dir.x == 0 || dir.y == 0 {
                if self.is_move_valid(pos, dir) {
                    neighbours.push((*pos + dir).unwrap());
                }
            } else {
                if diag {
                    if self.is_move_valid(pos, dir) {
                        neighbours.push((*pos + dir).unwrap());
                    }
                }
            }
        }
        neighbours
    }

    pub fn is_boundary(&self, pos: &Point) -> bool {
        pos.y == 0 || pos.y == self.len() - 1 || pos.x == 0 || pos.x == self[0].len()
    }
}

impl Deref for Grid {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0 // Deref returns a reference to the inner Vec<Vec<char>>
    }
}

impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for l in self.iter() {
            let row: String = l.iter().collect();
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Direction {
    pub y: i32,
    pub x: i32,
}

impl Direction {
    pub fn arrow_char_to_dir(c: &char) -> Option<Direction> {
        match c {
            '<' => Some(Direction { x: -1, y: 0 }),
            '^' => Some(Direction { x: 0, y: -1 }),
            '>' => Some(Direction { x: 1, y: 0 }),
            'v' => Some(Direction { x: 0, y: 1 }),
            _ => None,
        }
    }

    pub fn turn_left(self: &Self) -> Option<Direction> {
        match self {
            Direction { x: -1, y: 0 } => Some(Direction { x: 0, y: 1 }),
            Direction { x: 0, y: -1 } => Some(Direction { x: -1, y: 0 }),
            Direction { x: 1, y: 0 } => Some(Direction { x: 0, y: -1 }),
            Direction { x: 0, y: 1 } => Some(Direction { x: 1, y: 0 }),
            _ => None,
        }
    }

    pub fn turn_right(self: &Self) -> Option<Direction> {
        match self {
            Direction { x: -1, y: 0 } => Some(Direction { x: 0, y: -1 }),
            Direction { x: 0, y: -1 } => Some(Direction { x: 1, y: 0 }),
            Direction { x: 1, y: 0 } => Some(Direction { x: 0, y: 1 }),
            Direction { x: 0, y: 1 } => Some(Direction { x: -1, y: 0 }),
            _ => None,
        }
    }

    pub fn to_idx(self: &Self) -> Option<usize> {
        match self {
            Direction { x: -1, y: 0 } => Some(0),
            Direction { x: 0, y: -1 } => Some(1),
            Direction { x: 1, y: 0 } => Some(2),
            Direction { x: 0, y: 1 } => Some(3),
            Direction { x: -1, y: -1 } => Some(4),
            Direction { x: 1, y: -1 } => Some(5),
            Direction { x: -1, y: 1 } => Some(6),
            Direction { x: 1, y: 1 } => Some(7),
            _ => None,
        }
    }
}

impl Ord for Direction {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.y.cmp(&other.y) {
            Ordering::Equal => self.x.cmp(&other.x),
            other_order => other_order,
        }
    }
}

impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Direction {
    type Output = Direction; // Define the result type of the addition

    fn add(self, other: Direction) -> Direction {
        Direction {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Direction {
    type Output = Direction; // Define the result type of the addition

    fn sub(self, other: Direction) -> Direction {
        Direction {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Direction {
    type Output = Direction; // Define the result type of the addition

    fn mul(self, scalar: i32) -> Direction {
        Direction {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Add for Point {
    type Output = Point; // Define the result type of the addition

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Option<Point>; // Define the result type of the addition

    fn add(self, other: Direction) -> Option<Point> {
        let x = self.x as i32 + other.x;
        let y = self.y as i32 + other.y;
        if x >= 0 && y >= 0 {
            Some(Point {
                x: x as usize,
                y: y as usize,
            })
        } else {
            None
        }
    }
}

impl Sub<Direction> for Point {
    type Output = Option<Point>; // Define the result type of the addition

    fn sub(self, other: Direction) -> Option<Point> {
        let x = self.x as i32 - other.x;
        let y = self.y as i32 - other.y;
        if x >= 0 && y >= 0 {
            Some(Point {
                x: x as usize,
                y: y as usize,
            })
        } else {
            None
        }
    }
}

impl Add<&Direction> for Point {
    type Output = Option<Point>; // Define the result type of the addition

    fn add(self, other: &Direction) -> Option<Point> {
        let x = self.x as i32 + other.x;
        let y = self.y as i32 + other.y;
        if x >= 0 && y >= 0 {
            Some(Point {
                x: x as usize,
                y: y as usize,
            })
        } else {
            None
        }
    }
}

impl Sub<&Direction> for Point {
    type Output = Option<Point>; // Define the result type of the addition

    fn sub(self, other: &Direction) -> Option<Point> {
        let x = self.x as i32 - other.x;
        let y = self.y as i32 - other.y;
        if x >= 0 && y >= 0 {
            Some(Point {
                x: x as usize,
                y: y as usize,
            })
        } else {
            None
        }
    }
}

impl Mul<usize> for Point {
    type Output = Point; // Define the result type of the addition

    fn mul(self, scalar: usize) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare by x-coordinate first, then by y-coordinate
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            other_order => other_order,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
