use regex::Regex;

const INPUT: &str = include_str!("../../../inputs/day3.txt");

struct Instructions {
    instructions: Vec<(i64, i64)>,
}

impl Instructions {
    fn from_str(input: &str) -> Self {
        let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let matches: Vec<_> = regex
            .captures_iter(input)
            .map(|r| (r.get(1).unwrap().as_str(), r.get(2).unwrap().as_str()))
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect();

        Self {
            instructions: matches,
        }
    }

    fn mul_add(&self) -> i64 {
        self.instructions.iter().map(|(a, b)| a * b).sum()
    }
}

pub fn part1() {
    let inst = Instructions::from_str(INPUT);

    dbg!(inst.mul_add());
}

pub fn part2() {}

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
}
