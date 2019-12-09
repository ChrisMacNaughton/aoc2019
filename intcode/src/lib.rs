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
    pub memory: Vec<isize>,
    ip: usize,
}

impl Intcode {
    pub fn new(memory: Vec<isize>) -> Intcode {
        Intcode { memory, ip: 0 }
    }

    pub fn run(&mut self) {
        loop {
            // eprintln!("PC: {}", self.ip);
            let instruction = self.get(self.ip);
            // eprintln!("Instruction: {}", instruction);
            let mode1 = match instruction / 100 % 10 {
                1 => Mode::Immediate,
                _ => Mode::Position,
            };
            let mode2 = match instruction / 1_000 % 10 {
                1 => Mode::Immediate,
                _ => Mode::Position,
            };
            let opcode = instruction % 100;
            // eprintln!("Opcode: {}", opcode);
            match opcode {
                1 => self.add(mode1, mode2),
                2 => self.mul(mode1, mode2),
                3 => self.get_op(),
                4 => self.put_op(mode1),
                5 => self.jmp_true(mode1, mode2),
                6 => self.jmp_false(mode1, mode2),
                7 => self.lt(mode1, mode2),
                8 => self.eq(mode1, mode2),
                99 => return,
                // _ => panic!("Invalid opcode: {}", opcode),
                _ => {
                    println!("Invalid opcode {}", opcode);
                    self.step(1);
                }
            }
        }
    }

    pub fn get(&self, position: usize) -> isize {
        self.memory[position]
    }

    pub fn set(&mut self, position: usize, value: isize) {
        self.memory[position] = value;
    }

    fn arg(&mut self, offset: usize) -> isize {
        self.get(self.ip + offset)
    }

    fn step(&mut self, step: usize) {
        self.ip += step;
    }

    fn add(&mut self, mode1: Mode, mode2: Mode) {
        let pos1 = match mode1 {
            Mode::Position => {
                let pos = self.arg(1);
                self.get(pos as usize)
            }
            Mode::Immediate => self.arg(1),
        };
        let pos2 = match mode2 {
            Mode::Position => {
                let pos = self.arg(2);
                self.get(pos as usize)
            }
            Mode::Immediate => self.arg(2),
        };
        let destination = self.arg(3);
        // eprintln!("Set {} to {} + {}", destination, pos1, pos2);
        self.set(destination as usize, pos1 + pos2);
        self.step(4);
    }

    fn mul(&mut self, mode1: Mode, mode2: Mode) {
        let pos1 = match mode1 {
            Mode::Position => {
                let pos = self.arg(1);
                self.get(pos as usize)
            }
            Mode::Immediate => self.arg(1),
        };
        let pos2 = match mode2 {
            Mode::Position => {
                let pos = self.arg(2);
                self.get(pos as usize)
            }
            Mode::Immediate => self.arg(2),
        };
        let destination = self.arg(3);

        // eprintln!("Set {} to {} * {}", destination, pos1, pos2);
        self.set(destination as usize, pos1 * pos2);
        self.step(4);
    }

    fn get_op(&mut self) {
        let mut line = String::new();
        println!("INPUT: ");
        std::io::stdin().read_line(&mut line).unwrap();
        // line = line.trim();
        let destination = self.arg(1);

        // eprintln!("Set {} to {} (from INPUT)", destination, line);
        self.set(destination as usize, line.trim().parse().unwrap());
        self.step(2);
    }

    fn put_op(&mut self, mode: Mode) {
        let val = match mode {
            Mode::Position => {
                let pos = self.arg(1);
                self.get(pos as usize)
            }
            Mode::Immediate => self.arg(1),
        };
        println!("{}", val);
        self.step(2);
    }

    fn jmp_true(&mut self, mode1: Mode, mode2: Mode) {
        let val = match mode1 {
            Mode::Position => {
                let pos = self.arg(1);
                self.get(pos as usize)
            }
            Mode::Immediate => self.arg(1),
        };

        // eprintln!("Jump to {} if {} != 0", self.arg(2), val);
        if val != 0 {
            let pos = match mode2 {
                Mode::Position => {
                    let pos = self.arg(2);
                    self.get(pos as usize)
                }
                Mode::Immediate => self.arg(2),
            };
            self.ip = pos as usize;
        // self.ip = self.arg(2) as usize;
        } else {
            self.step(3);
        }
    }

    fn jmp_false(&mut self, mode1: Mode, mode2: Mode) {
        let val = match mode1 {
            Mode::Position => {
                let pos = self.arg(1);
                self.get(pos as usize)
            }
            Mode::Immediate => self.arg(1),
        };
        // eprintln!("Jump to {} if {} == 0", self.arg(2), val);
        if val == 0 {
            let pos = match mode2 {
                Mode::Position => {
                    let pos = self.arg(2);
                    self.get(pos as usize)
                }
                Mode::Immediate => self.arg(2),
            };
            self.ip = pos as usize;
        // self.ip = self.arg(2) as usize;
        } else {
            self.step(3);
        }
    }

    fn lt(&mut self, mode1: Mode, mode2: Mode) {
        let val1 = match mode1 {
            Mode::Position => {
                let pos = self.arg(1);
                self.get(pos as usize)
            }
            Mode::Immediate => self.arg(1),
        };
        let val2 = match mode2 {
            Mode::Position => {
                let pos = self.arg(2);
                self.get(pos as usize)
            }
            Mode::Immediate => self.arg(2),
        };

        let pos3 = self.arg(3);
        // eprintln!("set {} to 1 if {} < {}", pos3, val1, val2);
        if val1 < val2 {
            self.set(pos3 as usize, 1);
        } else {
            self.set(pos3 as usize, 0);
        }
        self.step(4);
    }

    fn eq(&mut self, mode1: Mode, mode2: Mode) {
        let val1 = match mode1 {
            Mode::Position => {
                let pos = self.arg(1);
                self.get(pos as usize)
            }
            Mode::Immediate => self.arg(1),
        };
        let val2 = match mode2 {
            Mode::Position => {
                let pos = self.arg(2);
                self.get(pos as usize)
            }
            Mode::Immediate => self.arg(2),
        };

        let pos3 = self.arg(3);
        // eprintln!("set {} to 1 if {} == {}", pos3, val1, val2);
        if val1 == val2 {
            self.set(pos3 as usize, 1);
        } else {
            self.set(pos3 as usize, 0);
        }
        self.step(4);
    }
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}
