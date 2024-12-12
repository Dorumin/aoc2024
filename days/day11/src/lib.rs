use std::{collections::HashMap, fmt::Write};

use cached::proc_macro::cached;

const INPUT: &str = include_str!("../../../inputs/day11.txt");

struct Stones {
    stones: Vec<Stone>,
}

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

    #[allow(unused)]
    fn tick_memo(&self, steps: u32) -> u64 {
        let mut buf = String::new();

        self.stones
            .iter()
            .map(|stone| tick_memo(stone.0, steps, &mut buf))
            .sum()
    }

    fn tick_nore(&self, steps: u32) -> u64 {
        let stones: Vec<_> = self.stones.iter().map(|stone| stone.0).collect();

        tick_memo_norecurse_goawaylemon(steps, &stones)
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

fn tick_memo_norecurse_goawaylemon(steps: u32, stones: &[u64]) -> u64 {
    let mut buf = String::new();
    let mut cache = HashMap::new();
    let mut stack = Vec::new();
    let mut total = 0;

    for stone in stones {
        stack.push((*stone, steps, 0, 0));

        while let Some((stone, steps, current, backref)) = stack.pop() {
            let stone_index = stack.len();

            if steps == 0 {
                stack[backref].2 += 1;

                continue;
            }

            if let Some(sum) = cache.get(&(stone, steps)) {
                stack[backref].2 += sum;

                continue;
            }

            if current != 0 {
                cache.insert((stone, steps), current);

                if stone_index == 0 {
                    total += current;
                } else {
                    stack[backref].2 += current;
                }

                continue;
            }

            // unpop
            stack.push((stone, steps, current, backref));

            write!(buf, "{}", stone).unwrap();

            if buf.len() % 2 == 0 {
                let left = buf[0..(buf.len() / 2)].parse().unwrap();
                let right = buf[(buf.len() / 2)..buf.len()].parse().unwrap();

                buf.clear();

                stack.push((right, steps - 1, 0, stone_index));
                stack.push((left, steps - 1, 0, stone_index));
            } else if stone == 0 {
                buf.clear();

                stack.push((1, steps - 1, 0, stone_index));
            } else {
                buf.clear();

                stack.push((stone * 2024, steps - 1, 0, stone_index));
            }
        }
    }

    total
}

#[cached(key = "(u64, u32)", convert = "{ (stone, steps) }")]
fn tick_memo(stone: u64, steps: u32, buf: &mut String) -> u64 {
    if steps == 0 {
        return 1;
    }

    write!(buf, "{}", stone).unwrap();

    if buf.len() % 2 == 0 {
        let left = buf[0..(buf.len() / 2)].parse().unwrap();
        let right = buf[(buf.len() / 2)..buf.len()].parse().unwrap();

        buf.clear();

        tick_memo(left, steps - 1, buf) + tick_memo(right, steps - 1, buf)
    } else if stone == 0 {
        buf.clear();

        tick_memo(1, steps - 1, buf)
    } else {
        buf.clear();

        tick_memo(stone * 2024, steps - 1, buf)
    }
}

pub fn part1() {
    let mut stones = Stones::from_str(INPUT);

    stones.tick(25);

    dbg!(stones.stones.len());
}

pub fn part2() {
    let stones = Stones::from_str(INPUT);

    dbg!(stones.tick_nore(75));
}
