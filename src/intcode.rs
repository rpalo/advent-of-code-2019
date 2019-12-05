use std::io;
use std::fs;

/// An Intcode Interpreter is a simple virtual machine that uses opcodes
/// to modify its internal memory
pub struct IntcodeInterpreter {
    memory: Vec<isize>,
    ip: usize,
}

impl IntcodeInterpreter {
    pub fn new(memory: Vec<isize>) -> Self {
        Self { memory, ip: 0}
    }

    /// Sets a memory address to a value
    pub fn set(&mut self, position: usize, value: isize) {
        //println!("Writing {} to {}", value, position);
        self.memory[position] = value;
    }

    /// Reads from a memory address
    pub fn get(&self, position: usize) -> isize {
        self.memory[position]
    }

    /// Shows the memory for debugging
    pub fn print(&self) {
        println!("{:?}", self.memory);
    }

    /// Get the current instruction
    pub fn current_instruction(&self) -> isize {
        self.get(self.ip) % 100
    }

    /// Runs the program in memory until the stopcode (99) is reached
    /// 
    /// All new ops should have their own method.
    /// They take no rust args, but read in args as needed and
    /// shift the instruction pointer when they're done.
    /// Steps should be the number of args used + 1 for the opcode
    pub fn run(&mut self) {
        loop {
            match self.current_instruction() {
                1 => self.op1(),
                2 => self.op2(),
                3 => self.op3(),
                4 => self.op4(),
                5 => self.op5(),
                6 => self.op6(),
                7 => self.op7(),
                8 => self.op8(),
                99 => return,
                _ => panic!("Unrecognized opcode {}.", self.get(self.ip)),
            };
        }
    }

    /// Reads a number from STDIN
    fn read_stdin() -> isize {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("STDIN read failed.");
        buffer.trim().parse::<isize>().unwrap()
    }

    /// Write a number to STDOUT
    fn write_stdout(number: isize) {
        println!("{}", number);
    }

    /// Process the parameter mode and provide the value given
    /// as a parameter
    fn arg(&self, offset: usize) -> isize {
        let new_index = (self.ip + offset) % self.memory.len();
        let mode = (self.memory[self.ip] / 10isize.pow(1 + offset as u32)) % 2;
        if mode == 1 {
            self.memory[new_index]
        } else if mode == 0 {
            self.get(self.memory[new_index] as usize)
        } else {
            panic!("Unknown parameter mode {}", mode);
        }
    }

    /// Returns the address to write output to
    fn output_address(&self, offset: usize) -> usize {
        let new_index = (self.ip + offset) % self.memory.len();
        self.memory[new_index] as usize
    }

    /// Steps the IP forward "count" steps, wrapping if needed
    fn step(&mut self, count: usize) {
        self.ip = (self.ip + count) % self.memory.len();
    }

    /// Add [1] + [2], store in [3]
    fn op1(&mut self) {
        let in1 = self.arg(1);
        let in2 = self.arg(2);
        let out = self.output_address(3);

        self.set(out, in1 + in2);
        
        self.step(4);
    }

    /// Mult [1] * [2], store in [3]
    fn op2(&mut self) {
        let in1 = self.arg(1);
        let in2 = self.arg(2);
        let out = self.output_address(3);

        self.set(out, in1 * in2);

        self.step(4);
    }

    /// Read one value from STDIN and store it in [1]
    fn op3(&mut self) {
        let out = self.output_address(1);

        self.set(out, Self::read_stdin());

        self.step(2);
    }

    /// Read [1] and send it to STDOUT
    fn op4(&mut self) {
        Self::write_stdout(self.arg(1));

        self.step(2);
    }

    /// If [1] != 0, set IP -> [2], else nothing
    fn op5(&mut self) {
        if self.arg(1) != 0 {
            self.ip = self.arg(2) as usize;
        } else {
            self.step(3);
        }
    }

    /// if [1] == 0, set IP -> [2], else nothing
    fn op6(&mut self) {
        if self.arg(1) == 0 {
            self.ip = self.arg(2) as usize;
        } else {
            self.step(3);
        }
    }

    /// if [1] < [2], set [3] to 1, else 0
    fn op7(&mut self) {
        let out = self.output_address(3);

        if self.arg(1) < self.arg(2) {
            self.set(out, 1);
        } else {
            self.set(out, 0);
        }

        self.step(4);
    }

    /// if [1] == [2], set [3] to 1, else 0
    fn op8(&mut self) {
        let out = self.output_address(3);

        if self.arg(1) == self.arg(2) {
            self.set(out, 1);
        } else {
            self.set(out, 0);
        }

        self.step(4);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_program1() {
        let program = vec![1, 0, 0, 0, 99];
        let mut a = IntcodeInterpreter::new(program);
        a.run();
        assert_eq!(2, a.get(0));
    }

    #[test]
    fn test_simple_program2() {
        let program = vec![2, 3, 0, 3, 99];
        let mut a = IntcodeInterpreter::new(program);
        a.run();
        assert_eq!(2, a.get(0));
        assert_eq!(6, a.get(3));
    }

    #[test]
    fn test_simple_program3() {
        let program = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut a = IntcodeInterpreter::new(program);
        a.run();
        assert_eq!(30, a.get(0));
    }
}