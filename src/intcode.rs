/// An Intcode Interpreter is a simple virtual machine that uses opcodes
/// to modify its internal memory
pub struct IntcodeInterpreter {
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