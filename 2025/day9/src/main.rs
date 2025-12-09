use std::collections::BinaryHeap;
use std::error::Error;
use std::ffi::OsString;
use std::ops::Add;
use std::time::Instant;
use std::{env, fs};

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Tile {
    x: i64,
    y: i64,
}

impl Tile {
    fn area(&self, b: &Tile) -> u64 {
        (b.x.abs_diff(self.x) + 1) * (b.y.abs_diff(self.y) + 1)
    }
}

impl Add<(i64, i64)> for Tile {
    type Output = Tile; // Define the result type of the addition

    fn add(self, scalar: (i64, i64)) -> Tile {
        Tile {
            x: self.x + scalar.0,
            y: self.y + scalar.1,
        }
    }
}

fn max_area(tiles: &Vec<Tile>, validate: bool) -> u64 {
    let mut heap: BinaryHeap<(u64, usize, usize)> = BinaryHeap::new();

    for i in 0..tiles.len() - 1 {
        for j in i + 1..tiles.len() {
            let area = tiles[i].area(&tiles[j]);
            heap.push((area, i, j));
        }
    }

    if !validate {
        if let Some((max, _i, _j)) = heap.pop() {
            return max;
        }
    } else {
        while !heap.is_empty() {
            if let Some((max, i, j)) = heap.pop() {
                let rect = [tiles[i], Tile{x: tiles[i].x, y: tiles[j].y}, tiles[j], Tile{x: tiles[j].x, y: tiles[i].y}];

                if rect_inside_polygon_inclusive(&rect, tiles) {
                    return max;
                }
            }
        }
    }

    0
}

// ===========================================================
// Integer geometry utilities
// ===========================================================

#[inline]
fn bbox_of_polygon(poly: &[Tile]) -> (i64, i64, i64, i64) {
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    for p in poly {
        if p.x < min_x { min_x = p.x; }
        if p.x > max_x { max_x = p.x; }
        if p.y < min_y { min_y = p.y; }
        if p.y > max_y { max_y = p.y; }
    }
    (min_x, min_y, max_x, max_y)
}

#[inline]
fn bbox_contains_rect(
    poly_bbox: (i64, i64, i64, i64),
    rect: &[Tile; 4]
) -> bool {
    let (px1, py1, px2, py2) = poly_bbox;

    rect.iter().all(|&p| {
        p.x >= px1 && p.x <= px2 && p.y >= py1 && p.y <= py2
    })
}


// ===========================================================
// 1) Point-in-polygon (inclusive) — Integer Ray Casting
// ===========================================================
/// Determine whether integer point `p` is inside polygon `poly`.
/// - `p` is (px, py)
/// - `poly` is a slice of (x, y) integer vertices (clockwise or ccw, may be open or closed)
/// - Points on the boundary (edges or vertices) are considered inside -> returns true.
/// - Uses only integer arithmetic (i64).
///
/// Complexity: O(n) time, O(1) extra space.
pub fn point_in_polygon_inclusive(p: &Tile, poly: &[Tile]) -> bool {
    let (px, py) = (p.x, p.y);
    let n = poly.len();
    if n < 3 {
        return false; // not a polygon
    }

    // Ensure polygon indexing handles last->first edge
    let mut inside = false;
    for i in 0..n {
        let (xi, yi) = (poly[i].x, poly[i].y);
        let (xj, yj) = (poly[(i + 1) % n].x, poly[(i + 1) % n].y);

        // 1) Check if point is exactly on the segment (xi,yi)-(xj,yj)
        // Collinearity test: (xj-xi)*(py-yi) == (yj-yi)*(px-xi)
        // And bounding-box test: px between xi..xj and py between yi..yj
        let dx = xj - xi;
        let dy = yj - yi;
        let dxp = px - xi;
        let dyp = py - yi;
        if dx * dyp == dy * dxp {
            // Collinear -> check if within segment bounds
            if (px >= xi.min(xj) && px <= xi.max(xj)) && (py >= yi.min(yj) && py <= yi.max(yj)) {
                return true; // point lies on an edge or vertex -> inclusive
            }
        }

        // 2) Standard ray-casting crossing test:
        // Count edges that cross horizontal ray to the right of the point.
        // We use the test "yi > py != yj > py" to identify edges that straddle the ray.
        if (yi > py) != (yj > py) {
            // Compute determinant to check relative position of intersection's x vs px:
            // det = (xj - xi)*(py - yi) - (px - xi)*(yj - yi)
            // if det == 0 -> ray intersects exactly at integer x = px -> on boundary -> true
            // else: toggle inside when (yj > yi) == (det > 0)
            let det = dx * dyp - dxp * dy;
            if det == 0 {
                // Ray hits the edge at exactly px -> treat as boundary -> inside
                return true;
            }
            // If y increases on the edge (yj > yi) and det > 0 then intersection is to the right of px.
            // If y decreases on the edge (yj < yi) and det < 0 then intersection is to the right of px.
            // The boolean equality captures that.
            if (yj > yi) == (det > 0) {
                inside = !inside;
            }
        }
    }

    inside
}

// ===========================================================
// 2) Segment intersection (inclusive, integer)
// ===========================================================

#[inline]
fn orient(a: Tile, b: Tile, c: Tile) -> i64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

pub fn segment_intersect_strict(
    a1: Tile, a2: Tile,
    b1: Tile, b2: Tile
) -> bool {
    let o1 = orient(a1, a2, b1);
    let o2 = orient(a1, a2, b2);
    let o3 = orient(b1, b2, a1);
    let o4 = orient(b1, b2, a2);

    // Strict intersection happens only when orientations differ strictly
    (o1 > 0 && o2 < 0 || o1 < 0 && o2 > 0)
    &&
    (o3 > 0 && o4 < 0 || o3 < 0 && o4 > 0)
}


// ===========================================================
// 3) Rectangle-inside-polygon test (fast + correct)
// ===========================================================

pub fn rect_inside_polygon_inclusive(
    rect: &[Tile; 4],  // 4 points in order
    poly: &[Tile]
) -> bool {
    if poly.len() < 3 {
        return false;
    }

    // ---- Step 1: Bounding-box fast reject ----
    let poly_bbox = bbox_of_polygon(poly);
    if !bbox_contains_rect(poly_bbox, rect) {
        return false;
    }

    // ---- Step 2: Corner tests ----
    for corner in rect.iter() {
        if !point_in_polygon_inclusive(corner, poly) {
            return false;
        }
    }

    // ---- Step 3: Edge intersection tests ----
    // Rectangle edges
    let rect_edges = [
        (rect[0], rect[1]),
        (rect[1], rect[2]),
        (rect[2], rect[3]),
        (rect[3], rect[0]),
    ];

    // Polygon edges
    let mut poly_edges = Vec::with_capacity(poly.len());
    for i in 0..poly.len() {
        poly_edges.push((poly[i], poly[(i + 1) % poly.len()]));
    }

    // Check for intersection
    for &(r1, r2) in &rect_edges {
        for &(p1, p2) in &poly_edges {
            // If touching counts as inside, change to strict intersection check.
            if segment_intersect_strict(r1, r2, p1, p2) {
                return false; // rectangle intersects polygon border
            }
        }
    }

    true
}


fn main() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;

    let start = Instant::now();

    let mut tiles = Vec::new();

    for line in fs::read_to_string(file_path)?.lines() {
        let mut ls = line.split(',');
        let t = Tile {
            x: ls.next().unwrap().parse::<i64>()?,
            y: ls.next().unwrap().parse::<i64>()?,
        };
        tiles.push(t);
    }

    //println!("{:?}", tiles);

    let duration = start.elapsed();

    println!(
        "Parse: {}s {}ms {}µs {}ns",
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();

    let ma = max_area(&tiles, false);

    let duration = start.elapsed();
    println!(
        "Part1: {} | {}s {}ms {}µs {}ns",
        ma,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    let start = Instant::now();

    let ma = max_area(&tiles, true);

    let duration = start.elapsed();
    println!(
        "Part2: {} | {}s {}ms {}µs {}ns",
        ma,
        duration.as_secs(),
        duration.subsec_millis(),
        duration.subsec_micros() % 1000,
        duration.subsec_nanos() % 1000
    );

    Ok(())
}
