use std::fmt::Write;

const INPUT: &str = include_str!("../../../inputs/day11.txt");

struct Stones {
    stones: Vec<Stone>,
}

#[derive(Debug)]
struct Stone(u64);

impl Stones {
    fn from_str(input: &str) -> Self {
        Self {
            stones: input
                .split_ascii_whitespace()
                .map(|n| Stone(n.parse().unwrap()))
                .collect(),
        }
    }

    fn tick(&mut self, steps: usize) {
        let mut stones = std::mem::take(&mut self.stones);
        let mut next = Vec::with_capacity(stones.len());

        let mut s = String::new();

        for i in 0..steps {
            eprintln!("step {i}, {} stones", stones.len());

            for stone in stones.iter() {
                stone.tick(&mut next, &mut s);
            }

            std::mem::swap(&mut next, &mut stones);

            next.clear();
        }

        self.stones = stones;
    }
}

impl Stone {
    fn tick(&self, stones: &mut Vec<Stone>, buf: &mut String) {
        write!(buf, "{}", self.0).unwrap();

        if buf.len() % 2 == 0 {
            let left = buf[0..(buf.len() / 2)].parse().unwrap();
            let right = buf[(buf.len() / 2)..buf.len()].parse().unwrap();

            stones.push(Stone(left));
            stones.push(Stone(right));
        } else if self.0 == 0 {
            stones.push(Stone(1));
        } else {
            stones.push(Stone(self.0 * 2024));
        }

        buf.clear();
    }
}

pub fn part1() {
    let mut stones = Stones::from_str(INPUT);

    stones.tick(25);

    dbg!(stones.stones.len());
}

pub fn part2() {
    let mut stones = Stones::from_str(INPUT);

    stones.tick(75);

    dbg!(stones.stones.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let thing = Stones::from_str("");
    }
}
