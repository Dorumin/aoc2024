use std::fmt::Write;

use cached::proc_macro::cached;

const INPUT: &str = include_str!("../../../inputs/day11.txt");

struct Stones {
    stones: Vec<Stone>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
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

    fn tick_memo(&self, steps: usize) -> usize {
        self.stones.iter().map(|stone| tick_memo(stone.0, steps)).sum()
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

#[cached]
fn tick_memo(stone: u64, steps: usize) -> usize {
    if steps == 0 {
        return 1;
    }

    let buf = stone.to_string();

    if buf.len() % 2 == 0 {
        let left = buf[0..(buf.len() / 2)].parse().unwrap();
        let right = buf[(buf.len() / 2)..buf.len()].parse().unwrap();

        tick_memo(left, steps - 1) + tick_memo(right, steps - 1)
    } else if stone == 0 {
        tick_memo(1, steps - 1)
    } else {
        tick_memo(stone * 2024, steps - 1)
    }
}

pub fn part1() {
    let mut stones = Stones::from_str(INPUT);

    stones.tick(25);

    dbg!(stones.stones.len());
}

pub fn part2() {
    let stones = Stones::from_str(INPUT);

    dbg!(stones.tick_memo(75));
}
