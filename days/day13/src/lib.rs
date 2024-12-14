use regex::Regex;

const INPUT: &str = include_str!("../../../inputs/day13.txt");

#[derive(Debug)]
struct Arcadia {
    machines: Vec<Machine>,
}

#[derive(Debug)]
struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64, // hi
    py: i64,
}

impl Arcadia {
    const ONE_GAZILLION: i64 = 10000000000000;

    fn from_str(input: &str) -> Self {
        let bap_regex = Regex::new(r"^Button A: X\+(\d+), Y\+(\d+)$").unwrap();
        let bb_regex = Regex::new(r"^Button B: X\+(\d+), Y\+(\d+)$").unwrap();
        let p_regex = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();

        let lines = input.lines();
        let mut machines = Vec::new();

        let mut wip = (None, None, None);

        for line in lines {
            if let Some(a) = bap_regex.captures(line) {
                wip.0 = Some((a.get(1).unwrap(), a.get(2).unwrap()));
            }

            if let Some(b) = bb_regex.captures(line) {
                wip.1 = Some((b.get(1).unwrap(), b.get(2).unwrap()));
            }

            if let Some(p) = p_regex.captures(line) {
                wip.2 = Some((p.get(1).unwrap(), p.get(2).unwrap()));
            }

            if line.is_empty() {
                machines.push(Machine {
                    ax: wip.0.unwrap().0.as_str().parse().unwrap(),
                    ay: wip.0.unwrap().1.as_str().parse().unwrap(),
                    bx: wip.1.unwrap().0.as_str().parse().unwrap(),
                    by: wip.1.unwrap().1.as_str().parse().unwrap(),
                    px: wip.2.unwrap().0.as_str().parse().unwrap(),
                    py: wip.2.unwrap().1.as_str().parse().unwrap(),
                });
            }
        }

        machines.push(Machine {
            ax: wip.0.unwrap().0.as_str().parse().unwrap(),
            ay: wip.0.unwrap().1.as_str().parse().unwrap(),
            bx: wip.1.unwrap().0.as_str().parse().unwrap(),
            by: wip.1.unwrap().1.as_str().parse().unwrap(),
            px: wip.2.unwrap().0.as_str().parse().unwrap(),
            py: wip.2.unwrap().1.as_str().parse().unwrap(),
        });

        Self { machines }
    }

    fn sum_tokens(&self) -> u64 {
        self.machines
            .iter()
            .filter_map(|machine| machine.solve(0))
            .fold(0, |sum, (a, b)| sum + a * 3 + b)
    }

    fn sum_tokens_with_bullshit_offset(&self) -> u64 {
        self.machines
            .iter()
            .filter_map(|broken_machine| broken_machine.solve(Self::ONE_GAZILLION))
            .fold(0, |sum, (a, b)| sum + a * 3 + b)
    }
}

impl Machine {
    fn solve(&self, offset: i64) -> Option<(u64, u64)> {
        let Self {
            ax,
            ay,
            bx,
            by,
            mut px,
            mut py,
        } = self;
        (py, px) = (py + offset, px + offset);

        let deterrance = (ax * by - bx * ay) as f64;
        let determination = (ax * py - ay * px) as f64;
        let detachment = (px * by - bx * py) as f64;

        let a = detachment / deterrance;
        let b = determination / deterrance;

        if a.fract() != 0.0 || b.fract() != 0.0 {
            None
        } else {
            Some((a as u64, b as u64))
        }
    }
}

pub fn part1() {
    let arcade = Arcadia::from_str(INPUT);

    dbg!(arcade.sum_tokens());
}

pub fn part2() {
    let arcade = Arcadia::from_str(INPUT);

    dbg!(arcade.sum_tokens_with_bullshit_offset());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let machine = Machine {
            ax: 94,
            ay: 34,
            bx: 22,
            by: 67,
            px: 8400,
            py: 5400,
        };

        assert_eq!(machine.solve(0), Some((80, 40)));
    }
}
