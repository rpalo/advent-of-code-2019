/// Day 1: The Tyranny of the Rocket Equation
/// 
/// Calculate the amount of fuel required to launch your spaceship!

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Calculate the fuel required to lift one module, based on its mass
fn fuel_required(mass: &usize) -> usize {
    (mass / 3) - 2
}

/// Calculates the fuel required to lift one module, but factors in the
/// weight of the fuel as well
fn recursive_fuel_required(mass: &usize) -> usize {
    if *mass <= 6 {
        0
    } else {
        let next_mass = (mass / 3) - 2;
        next_mass + recursive_fuel_required(&next_mass)
    }
}

/// Calculate the total fuel requirements for the launch
fn fuel_requirements(module_masses: Vec<usize>) -> usize {
    module_masses.iter().map(recursive_fuel_required).sum()
}


/// Parses the input file, which is a bunch of numbers, one per line
fn parse_input() -> Vec<usize> {
    let buf = BufReader::new(File::open("data/day1.txt").unwrap());
    buf.lines()
        .map(|result| result.unwrap())
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

/// Main day 1 code
pub fn run() {
    let data = parse_input();
    let total_requirements = fuel_requirements(data);
    println!("Total fuel required: {}", total_requirements);
}