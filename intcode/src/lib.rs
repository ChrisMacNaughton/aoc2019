use std::convert::TryFrom;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_example_1() {
        // (1 + 1 = 2)
        let input = [1, 0, 0, 0, 99];
        let expected = [2, 0, 0, 0, 99];
        let mut intcode = Intcode::new(input.to_vec());
        intcode.run();
        assert_eq!(intcode.memory, expected);
    }

    #[test]
    fn validate_example_2() {
        // (3 * 2 = 6)
        let input = [2, 3, 0, 3, 99];
        let expected = [2, 3, 0, 6, 99];
        let mut intcode = Intcode::new(input.to_vec());
        intcode.run();
        assert_eq!(intcode.memory, expected);
    }

    #[test]
    fn validate_example_3() {
        // (99 * 99 = 9801)
        let input = [2, 4, 4, 5, 99, 0];
        let expected = [2, 4, 4, 5, 99, 9801];
        let mut intcode = Intcode::new(input.to_vec());
        intcode.run();
        assert_eq!(intcode.memory, expected);
    }

    #[test]
    fn validate_example_4() {
        let input = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        let expected = [30, 1, 1, 4, 2, 5, 6, 0, 99];
        let mut intcode = Intcode::new(input.to_vec());
        intcode.run();
        assert_eq!(intcode.memory, expected);
    }
}

pub struct Intcode {
    pub memory: Vec<usize>,
    ip: usize,
}

impl Intcode {
    pub fn new(memory: Vec<usize>) -> Intcode {
        Intcode { memory, ip: 0 }
    }

    pub fn run(&mut self) {
        loop {
            match self.current_op() {
                OpCode::Add => self.add(),
                OpCode::Mul => self.mul(),
                OpCode::End => return,
            }
        }
    }

    fn current_op(&self) -> OpCode {
        OpCode::try_from(&self.get(self.ip))
            .expect(&format!("Incalid opcode ({})!", self.get(self.ip)))
    }

    pub fn get(&self, position: usize) -> usize {
        self.memory[position]
    }

    pub fn set(&mut self, position: usize, value: usize) {
        self.memory[position] = value;
    }

    fn arg(&mut self, offset: usize) -> usize {
        self.get(self.ip + offset)
    }

    fn step(&mut self, step: usize) {
        self.ip += step;
    }

    fn add(&mut self) {
        let pos1 = self.arg(1);
        let pos2 = self.arg(2);
        let destination = self.arg(3);

        self.set(destination, self.get(pos1) + self.get(pos2));
        self.step(4);
    }

    fn mul(&mut self) {
        let pos1 = self.arg(1);
        let pos2 = self.arg(2);
        let destination = self.arg(3);

        self.set(destination, self.get(pos1) * self.get(pos2));
        self.step(4);
    }
}

#[derive(Debug)]
enum OpCode {
    Add,
    Mul,
    End,
}

impl TryFrom<&usize> for OpCode {
    type Error = String;

    fn try_from(input: &usize) -> Result<OpCode, Self::Error> {
        match input {
            1 => Ok(OpCode::Add),
            2 => Ok(OpCode::Mul),
            99 => Ok(OpCode::End),
            _ => Err(format!("Invalid OpCode: {}", input)),
        }
    }
}
