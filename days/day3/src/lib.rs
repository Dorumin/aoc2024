use regex::{Captures, Regex};

const INPUT: &str = include_str!("../../../inputs/day3.txt");

struct Instructions {
    instructions: Vec<Instruction>,
}

enum Instruction {
    Do,
    Dont,
    Mul { a: i64, b: i64 },
}

impl Instruction {
    fn from_captures(c: &Captures) -> Self {
        // Gross. But I can't use captures for the prefix without messing up the parser
        let f = c.get(0).unwrap();
        if f.as_str().starts_with("don't") {
            return Self::Dont;
        } else if f.as_str().starts_with("do") {
            return Self::Do;
        }

        let a = c.get(1).unwrap().as_str().parse().unwrap();
        let b = c.get(2).unwrap().as_str().parse().unwrap();

        Self::Mul { a, b }
    }
}

impl Instructions {
    fn from_str(input: &str) -> Self {
        let regex = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();

        let instructions: Vec<_> = regex
            .captures_iter(input)
            .map(|r| Instruction::from_captures(&r))
            .collect();

        Self { instructions }
    }

    fn mul_add(&self) -> i64 {
        self.instructions
            .iter()
            .filter_map(|ins| {
                if let Instruction::Mul { a, b } = ins {
                    Some(a * b)
                } else {
                    None
                }
            })
            .sum()
    }

    fn mul_add_enabled(&self) -> i64 {
        let mut enabled = true;
        self.instructions
            .iter()
            .filter_map(|ins| match ins {
                Instruction::Do => {
                    enabled = true;
                    None
                }
                Instruction::Dont => {
                    enabled = false;
                    None
                }
                Instruction::Mul { a, b } if enabled => Some(a * b),
                _ => None,
            })
            .sum()
    }
}

pub fn part1() {
    let inst = Instructions::from_str(INPUT);

    dbg!(inst.mul_add());
}

pub fn part2() {
    let inst = Instructions::from_str(INPUT);

    dbg!(inst.mul_add_enabled());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo_uno() {
        let inst = Instructions::from_str(
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        );

        assert_eq!(inst.mul_add(), 161);
    }

    #[test]
    fn ejemplo_dos() {
        let inst = Instructions::from_str(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );

        assert_eq!(inst.mul_add_enabled(), 48);
    }
}
