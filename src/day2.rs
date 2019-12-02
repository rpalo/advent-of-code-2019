/// Day 2: 1202 Program Alarm
/// 
/// Create an "Intcode" computer that can calculate a gravity slingshot

use std::fs;

use crate::intcode::IntcodeInterpreter;

/// Parses the input.  Expects a single line of integers separated by
/// commas
fn parse_input() -> Vec<usize> {
    let text: String = fs::read_to_string("data/day2.txt").unwrap();
    let cleaned = text.trim();
    cleaned.split(",").map( |c| c.parse().unwrap()).collect()
}

/// Given a desired output, hunts through the possible values of position
/// 1 and 2 (termed "noun" and "verb") by brute force until the output
/// is found.
fn find_output(output: usize, ints: Vec<usize>) -> (usize, usize) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut interpreter = IntcodeInterpreter::new(ints.clone());
            interpreter.set(1, noun);
            interpreter.set(2, verb);
            interpreter.run();
            if interpreter.get(0) == output {
                return (noun, verb);
            }
        }
    }
    
    panic!("Couldn't find output!");
}

pub fn run() {
    let ints = parse_input();

    println!("Part 1:");
    let mut interpreter = IntcodeInterpreter::new(ints.clone());
    interpreter.set(1, 12);
    interpreter.set(2, 2);
    interpreter.run();
    println!("After running, position 0 is: {}", interpreter.get(0));

    println!("Part 2:");
    let (noun, verb) = find_output(19690720, ints.clone());
    println!("Noun: {}, Verb: {}", noun, verb);
    println!("Secret code is: {}{}", noun, verb);

}