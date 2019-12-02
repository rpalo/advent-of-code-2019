/// An Intcode Interpreter is a simple virtual machine that uses opcodes
/// to modify its internal memory
pub struct IntcodeInterpreter {
    memory: Vec<usize>,
    ip: usize,
}

impl IntcodeInterpreter {
    pub fn new(memory: Vec<usize>) -> Self {
        Self { memory, ip: 0}
    }

    /// Sets a memory address to a value
    pub fn set(&mut self, position: usize, value: usize) {
        self.memory[position] = value;
    }

    /// Reads from a memory address
    pub fn get(&self, position: usize) -> usize {
        self.memory[position]
    }

    /// Shows the memory for debugging
    pub fn print(&self) {
        println!("{:?}", self.memory);
    }

    /// Get the current instruction
    pub fn current_instruction(&self) -> usize {
        self.get(self.ip)
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
                99 => return,
                _ => panic!("Unrecognized opcode {}.", self.get(self.ip)),
            };
        }
    }

    /// Get the value at "offset" index after ip
    fn arg(&self, offset: usize) -> usize {
        let new_index = (self.ip + offset) % self.memory.len();
        self.memory[new_index]
    }

    /// Steps the IP forward "count" steps, wrapping if needed
    fn step(&mut self, count: usize) {
        self.ip = (self.ip + count) % self.memory.len();
    }

    /// Add [1] + [2], store in [3]
    fn op1(&mut self) {
        let in1 = self.arg(1);
        let in2 = self.arg(2);
        let out = self.arg(3);

        self.set(out, self.get(in1) + self.get(in2));
        
        self.step(4)
    }

    /// Mult [1] * [2], store in [3]
    fn op2(&mut self) {
        let in1 = self.arg(1);
        let in2 = self.arg(2);
        let out = self.arg(3);

        self.set(out, self.get(in1) * self.get(in2));

        self.step(4)
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