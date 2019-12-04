/// Day 3: Crossed Wires
/// 
/// Figure out which directions wires go and where they cross

use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs;
use std::ops::Add;

/// A coordinate is a 2D location (or Vector change!) with X and Y components
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coordinate {
    x: isize,
    y: isize,
}

/// You can Add Coordinates together with + to get a new one
impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Self {
        Self {x, y}
    }
}

/// A Wire is a chain of coordinates that are electrically connected
type Wire = Vec<Coordinate>;

/// Moves are an ordered list of delta moves to make to form a wire
type Moves = Vec<Coordinate>;

/// Expects two lines of comma separated move strings, one for each wire.
/// The move strings are of the pattern [UDLR][0-9]+
///     [UDLR]: Specifies whether the wire is going up, down, left or right
///     [0-9]+: Specifies how many spaces that leg of the wire covers
/// 
/// Returns both processed wires
fn parse_input() -> (Wire, Wire) {
    let text = fs::read_to_string("data/day3.txt").unwrap();
    let lines: Vec<&str> = text.split("\n").collect();
    let mut wires = lines.into_iter().map(build_moves).map(make_wire);
    (wires.next().unwrap(), wires.next().unwrap())
}

/// Builds a list of Moves out of an input comma separated string as described
/// above.
fn build_moves(text_moves: &str) -> Moves {
    let mut results: Vec<Coordinate> = Vec::new();
    
    
    for step in text_moves.split(",") {
        let (direction, count_text) = step.split_at(1);
        let count: usize = count_text.parse().unwrap();
        let move_coord = if direction == "U" {
            Coordinate::new(0, 1)
        } else if direction == "D" {
            Coordinate::new(0, -1)
        } else if direction == "L" {
            Coordinate::new(-1, 0)
        } else if direction == "R" {
            Coordinate::new(1, 0)
        } else {
            panic!("Weird step {}", direction);
        };

        for _ in 0..count {
            results.push(move_coord);
        }
    }

    results
}


/// Build a Wire out of relative Moves
fn make_wire(moves: Moves) -> Wire {
    let mut current = Coordinate { x: 0, y: 0 };
    let mut results: Wire = Vec::new();

    for step in moves {
        current = step + current;
        results.push(current);
    }

    results
}

/// Calculate a Coordinate's absolute distance from the origin
fn origin_manhattan_distance(coord: &Coordinate) -> usize {
    (coord.x.abs() + coord.y.abs()) as usize
}

/// Given two Wires, find the location where they cross that is closest to the
/// origin (which is where both wires start, and doesn't count as a cross)
fn find_closest_cross(a: &Wire, b: &Wire) -> Coordinate {
    let a_set: HashSet<&Coordinate> = HashSet::from_iter(a.iter());
    let b_set: HashSet<&Coordinate> = HashSet::from_iter(b.iter());
    **a_set.intersection(&b_set).min_by_key(|c| origin_manhattan_distance(c)).unwrap()
}

/// Find the first occurrence of a Coordinate in a Wire (index-wise, but 1-based)
fn find_in_wire(wire: &Wire, target: &Coordinate) -> usize {
    wire.iter().position(|e| e == target).unwrap() + 1
}

/// Find the shortest distance you can travel on each wire (summed) before you
/// hit a cross.
fn shortest_cross_distance(a: &Wire, b: &Wire) -> usize {
    let a_set: HashSet<&Coordinate> = HashSet::from_iter(a.iter());
    let b_set: HashSet<&Coordinate> = HashSet::from_iter(b.iter());
    a_set.intersection(&b_set).map(|c| find_in_wire(a, c) + find_in_wire(b, c)).min().unwrap()
}

/// Main Day 3 logic to solve the puzzles
pub fn run() {
    let (a, b) = parse_input();
    let closest_cross = find_closest_cross(&a, &b);
    println!("Closest cross distance is {}", origin_manhattan_distance(&closest_cross));
    println!("Fewest combined steps: {}", shortest_cross_distance(&a, &b));
}