/// Day 2: 1202 Program Alarm
/// 
/// Create an "Intcode" computer that can calculate a gravity slingshot

use std::fs;

/// Parses the input.  Expects a single line of integers separated by
/// commas
fn parse_input() -> Vec<usize> {
    let text: String = fs::read_to_string("data/day2.txt").unwrap();
    let cleaned = text.trim();
    cleaned.split(",").map( |c| c.parse().unwrap()).collect()
}

/// An Intcode Interpreter is a simple virtual machine that uses opcodes
/// to modify its internal memory
struct IntcodeInterpreter {
    ints: Vec<usize>,
    ip: usize,
}

impl IntcodeInterpreter {
    pub fn new(ints: Vec<usize>) -> Self {
        Self { ints, ip: 0}
    }

    /// Sets a memory address to a value
    pub fn set(&mut self, position: usize, value: usize) {
        self.ints[position] = value;
    }

    /// Reads from a memory address
    pub fn get(&self, position: usize) -> usize {
        self.ints[position]
    }

    /// Runs the program in memory until the stopcode (99) is reached
    /// (To be refactored and generalized)
    pub fn run(&mut self) {
        while self.ints[self.ip] != 99 {
            let opcode = self.ints[self.ip];
            let in1 = self.ints[self.ip + 1];
            let in2 = self.ints[self.ip + 2];
            let out = self.ints[self.ip + 3];

            if opcode == 1 {
                self.ints[out] = self.ints[in1] + self.ints[in2];
            } else if opcode == 2 {
                self.ints[out] = self.ints[in1] * self.ints[in2];
            } else {
                panic!("Unrecognized opcode {}!", opcode);
            }
            self.ip = (self.ip + 4) % self.ints.len();
        }
    }
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