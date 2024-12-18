use std::str::Lines;

const INPUT: &str = include_str!("../../../inputs/day17.txt");

#[derive(Debug, Clone)]
struct Program {
    pointer: usize,
    registers: Registers,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
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
            _ => unsafe { std::hint::unreachable_unchecked() },
        })
    }

    fn tick(&mut self, output: &mut Vec<u8>) -> Option<()> {
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

                output.push(comboni as u8);
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
            _ => unsafe { std::hint::unreachable_unchecked() },
        }

        Some(())
    }

    fn process(&mut self, output: &mut Vec<u8>) {
        while self.tick(output).is_some() {}
    }

    fn find_quine(&mut self) -> u64 {
        let mut output = Vec::new();

        // Given pux's program analysis, only three bits at a time matter
        // So we can """brute force""" 3 bit numbers at a time
        let mut triptetmocoquecahedrons = vec![0; self.instructions.len()];
        let mut i = triptetmocoquecahedrons.len() - 1;

        loop {
            if triptetmocoquecahedrons[i] > 7 {
                triptetmocoquecahedrons[i] = 0;
                i += 1;
                triptetmocoquecahedrons[i] += 1;

                continue;
            }

            // We can shift right-to-left the currently resolved digits
            // and they will add up to an exploratory register that we
            // can use to see if our prefix will work to generate the next instruction
            let register = triptetmocoquecahedrons
                .iter()
                .rev()
                .take(self.instructions.len() - i)
                .fold(0, |register, offset| (register << 3) + offset);

            output.clear();
            self.pointer = 0;
            self.registers.a = register;

            while self.tick(&mut output).is_some() {}

            let suffix_lines_up = output.len() == self.instructions.len() - i
                && output
                    .iter()
                    .zip(self.instructions.iter().skip(i))
                    .all(|(o, i)| *o == i.0);

            if suffix_lines_up && i == 0 {
                return register;
            }

            if suffix_lines_up {
                assert_ne!(i, 0);
                i -= 1;
            } else {
                triptetmocoquecahedrons[i] += 1;
            }
        }
    }
}

pub fn part1() {
    let mut program = Program::from_str(INPUT);
    let mut output = Vec::new();

    program.process(&mut output);

    let output = output
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");

    dbg!(output);
}

pub fn part2() {
    let mut program = Program::from_str(INPUT);

    let quindex = program.find_quine();

    dbg!(quindex);
}

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
