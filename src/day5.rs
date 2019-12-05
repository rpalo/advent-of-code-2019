/// Day 5: Sunny with a Chance of Asteroids
/// 
/// Test out the Thermal Environment Supervision Terminal
/// in the Intcode Interpreter

use std::fs;
use crate::intcode;

/// Parses the input.  Expects a single line of integers separated by
/// commas
fn parse_input() -> Vec<isize> {
    let text: String = fs::read_to_string("data/day5.txt").unwrap();
    let cleaned = text.trim();
    cleaned.split(",").map( |c| c.parse().unwrap()).collect()
}

pub fn run() {
    let ints = parse_input();
    let mut computer = intcode::IntcodeInterpreter::new(ints);
    computer.run();
}