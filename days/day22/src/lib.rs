const INPUT: &str = include_str!("../../../inputs/day22.txt");

struct MonkeyBusiness {
    monkeys: Vec<u64>,
}

fn monkey_tick(n: u64) -> u64 {
    let s = n;
    let s = prune(mix(s, s * 64));
    let s = prune(mix(s, s / 32));
    let s = prune(mix(s, s * 2048));

    s
}

fn mix(n: u64, s: u64) -> u64 {
    n ^ s
}

fn prune(s: u64) -> u64 {
    s % 16777216
}

impl MonkeyBusiness {
    fn from_str(input: &str) -> Self {
        Self {
            monkeys: input.lines().map(|line| line.parse().unwrap()).collect(),
        }
    }

    fn tick_all(&mut self, times: usize) {
        for monki in self.monkeys.iter_mut() {
            *monki = (0..times).fold(*monki, |m, _| monkey_tick(m));
        }
    }

    fn sum(&self) -> u64 {
        self.monkeys.iter().sum()
    }
}

pub fn part1() {
    let mut market = MonkeyBusiness::from_str(INPUT);

    market.tick_all(2000);

    dbg!(market.sum());
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut market = MonkeyBusiness::from_str(
            "1
10
100
2024",
        );

        market.tick_all(2000);

        assert_eq!(market.sum(), 37327623);
    }
}
