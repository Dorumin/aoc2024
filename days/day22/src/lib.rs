use std::collections::HashMap;

const INPUT: &str = include_str!("../../../inputs/day22.txt");

struct MonkeyBusiness {
    monkeys: Vec<u64>,
}

fn monkey_tick(n: u64) -> u64 {
    let s = n;
    let s = prune(mix(s, s * 64));
    let s = prune(mix(s, s / 32));

    prune(mix(s, s * 2048))
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

    // don't inline to aid in perf measurements
    #[inline(never)]
    #[allow(unused)]
    fn optimize_buy_sequence<const N: usize>(&self, times: usize) -> ([i8; N], u64) {
        let mut map = HashMap::new();

        for (monki_index, monki) in self.monkeys.iter().enumerate() {
            let mut monki = *monki;
            let mut seq = [0; N];
            let mut last_nana_price = (monki % 10) as i8;

            for i in 0..times {
                let nana_price = (monki % 10) as i8;
                seq[0] = nana_price - last_nana_price;
                seq.rotate_left(1);

                // Skip first *4* inserts, to get valid deltas
                // as first insert will always be 0 (invalid self-delta)
                if i >= N {
                    // We can initialize our payouts with -1 as a sentinel for "unmatched"
                    let prices = map.entry(seq).or_insert_with(|| vec![-1; self.monkeys.len()]);

                    if prices[monki_index] == -1 {
                        prices[monki_index] = nana_price;
                    }
                }

                monki = monkey_tick(monki);
                last_nana_price = nana_price;
            }
        }

        let sequence_prices: Vec<_> = map
            .into_iter()
            .map(|(seq, prices)| {
                (
                    seq,
                    // don't add up the -1s
                    prices.iter().filter(|n| **n >= 0).map(|n| *n as u64).sum::<u64>(),
                )
            })
            .collect();

        sequence_prices.into_iter().max_by_key(|(_, price)| *price).unwrap()
    }

    // don't inline to aid in perf measurements
    #[inline(never)]
    fn lemonize_buy_sequence<const N: usize>(&self, times: usize) -> ([i8; N], u64) {
        const POSSIBLE_DELTA_VALUES: usize = 19;

        // Can't be const, curiously
        let linear_backing_size = POSSIBLE_DELTA_VALUES.pow(N as u32);

        let mut linear_backing = vec![0; linear_backing_size];
        let mut witnessed_sequence_ids = vec![false; linear_backing_size];
        let mut best_seq_so_far = [0; N];
        let mut best_sales_so_far = 0;

        for monki in self.monkeys.iter() {
            witnessed_sequence_ids.fill(false);

            let mut monki = *monki;
            let mut seq = [0; N];
            let mut last_nana_price = (monki % 10) as i8;

            // Priming the sequence
            for _ in 0..N {
                let nana_price = (monki % 10) as i8;
                seq[0] = nana_price - last_nana_price;
                seq.rotate_left(1);

                monki = monkey_tick(monki);
                last_nana_price = nana_price;
            }

            for _ in 0..(times - N) {
                let nana_price = (monki % 10) as i8;
                seq[0] = nana_price - last_nana_price;
                seq.rotate_left(1);

                // We calculate every component of `seq` into the id at each step
                // Might be able to optimize this to a sum like pux, but this fn
                // also returns the specific best delta seq as well as best payout
                let seq_id = seq.iter().enumerate().fold(0, |sum, (index, delta)| {
                    sum + (*delta + 9) as usize * POSSIBLE_DELTA_VALUES.pow((N - index - 1) as u32)
                });

                // branching on price != 0 is negligible
                if !witnessed_sequence_ids[seq_id] {
                    witnessed_sequence_ids[seq_id] = true;
                    linear_backing[seq_id] += nana_price as u64;

                    if linear_backing[seq_id] > best_sales_so_far {
                        best_seq_so_far = seq;
                        best_sales_so_far = linear_backing[seq_id]
                    }
                }

                monki = monkey_tick(monki);
                last_nana_price = nana_price;
            }
        }

        (best_seq_so_far, best_sales_so_far)
    }
}

pub fn part1() {
    let mut market = MonkeyBusiness::from_str(INPUT);

    market.tick_all(2000);

    dbg!(market.sum());
}

pub fn part2() {
    let market = MonkeyBusiness::from_str(INPUT);

    // let (optimal_sequence, optimal_price) = market.optimize_buy_sequence::<4>(2000);

    // eprintln!("optimal prices for optimal sequence {optimal_sequence:?} is {optimal_price}");

    let (optimal_sequence, optimal_price) = market.lemonize_buy_sequence::<4>(2000);

    eprintln!("lemonal prices for optimal sequence {optimal_sequence:?} is {optimal_price}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut market = MonkeyBusiness::from_str(
            "\
1
10
100
2024",
        );

        market.tick_all(2000);

        assert_eq!(market.sum(), 37327623);
    }

    #[test]
    fn example_two() {
        // don't put the same input as example one lmao
        let market = MonkeyBusiness::from_str(
            "\
1
2
3
2024",
        );

        let (optimal_sequence, optimal_price) = market.optimize_buy_sequence::<4>(2000);

        assert_eq!(optimal_sequence, [-2, 1, -1, 3]);
        assert_eq!(optimal_price, 23);
    }
}
