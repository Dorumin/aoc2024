use std::str::Lines;

const INPUT: &str = include_str!("../../../inputs/day17.txt");

#[derive(Debug)]
struct Program {
    pointer: usize,
    registers: Registers,
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct Registers {
    // Registers can hold "any number", let's assume 64 bits are enough
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug, Clone, Copy)]
struct Instruction(u8);

impl Registers {
    fn from_lines(lines: &mut Lines) -> Registers {
        let mut a = None;
        let mut b = None;
        let mut c = None;

        for line in lines {
            if line.is_empty() {
                break;
            }

            let line = line.strip_prefix("Register ").unwrap();
            let (register, line) = strip(line, |c| c.is_ascii_alphabetic()).unwrap();
            let line = line.strip_prefix(": ").unwrap();
            let (value, _) = strip(line, int_matcher()).unwrap();
            let value = value.parse().unwrap();

            match register {
                "A" => a = Some(value),
                "B" => b = Some(value),
                "C" => c = Some(value),
                _ => unreachable!(),
            }
        }

        Self {
            a: a.unwrap(),
            b: b.unwrap(),
            c: c.unwrap(),
        }
    }
}

impl Instruction {
    fn new(n: u8) -> Self {
        if n > 7 {
            panic!("It's... it's too big, it won't fit");
        }

        Self(n)
    }

    fn parse(s: &str) -> Self {
        let n = s.parse().unwrap();

        Self::new(n)
    }
}

impl Program {
    fn from_str(input: &str) -> Self {
        let mut lines = input.lines();
        let registers = Registers::from_lines(&mut lines);
        let program = lines.next().unwrap().strip_prefix("Program: ").unwrap();
        let instructions = program.split(",").map(Instruction::parse).collect();

        Self {
            pointer: 0,
            registers,
            instructions,
        }
    }

    fn read_instruction(&mut self) -> Option<Instruction> {
        let ins = self.instructions.get(self.pointer)?;

        self.pointer += 1;

        Some(*ins)
    }

    fn read_literal(&mut self) -> Option<u8> {
        let ins = self.read_instruction()?;

        Some(ins.0)
    }

    fn read_combo(&mut self) -> Option<u64> {
        let ins = self.read_instruction()?;

        Some(match ins.0 {
            0..=3 => ins.0 as u64,
            4 => self.registers.a,
            5 => self.registers.b,
            6 => self.registers.c,
            _ => unreachable!(),
        })
    }

    fn tick(&mut self) -> Option<()> {
        let ins = self.read_instruction()?;

        match ins {
            // adv
            Instruction(0) => {
                let numerator = self.registers.a;
                let denominator = 2u64.pow(self.read_combo()? as u32);

                // truncate
                self.registers.a = numerator / denominator;
            }
            // bxl
            Instruction(1) => {
                let mask = self.read_literal()? as u64;

                self.registers.b ^= mask;
            }
            // bst
            Instruction(2) => {
                let truncated = self.read_combo()? % 8;

                self.registers.b = truncated;
            }
            // jnz
            Instruction(3) => {
                // Always read 2 instructions if 0 or not
                let literal = self.read_literal()?;

                if self.registers.a != 0 {
                    self.pointer = literal as usize;
                }
            }
            // bxc
            Instruction(4) => {
                // For legacy reasons, read operand and drop it
                let _shitty_arch = self.read_instruction()?;

                self.registers.b ^= self.registers.c;
            }
            // out
            Instruction(5) => {
                let comboni = self.read_combo()? % 8;

                eprint!("{comboni},");
            }
            // bdv
            Instruction(6) => {
                let numerator = self.registers.a;
                let denominator = 2u64.pow(self.read_combo()? as u32);

                // truncate
                self.registers.b = numerator / denominator;
            }
            // cdv
            Instruction(7) => {
                let numerator = self.registers.a;
                let denominator = 2u64.pow(self.read_combo()? as u32);

                // truncate
                self.registers.c = numerator / denominator;
            }
            _ => unreachable!(),
        }

        Some(())
    }

    fn process(&mut self) {
        while self.tick().is_some() {}
    }
}

pub fn part1() {
    let mut program = Program::from_str(INPUT);

    program.process();

    dbg!(program);
}

pub fn part2() {}

fn strip(hays: &str, matcher: impl FnMut(char) -> bool) -> Option<(&str, &str)> {
    let rest = hays.trim_start_matches(matcher);
    let stripped = &hays[0..(hays.len() - rest.len())];

    if stripped.is_empty() {
        return None;
    }

    Some((stripped, rest))
}

fn int_matcher() -> impl FnMut(char) -> bool {
    let mut index = 0;

    move |c| {
        let matched = (index == 0 && c == '-') || c.is_ascii_digit();
        index += 1;

        matched
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let thing = Program::from_str("");
    }
}
