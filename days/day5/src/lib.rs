use std::collections::HashMap;

const INPUT: &str = include_str!("../../../inputs/day5.txt");

struct Update {
    ordering_rules: OrderingRules,
    pages: Vec<Vec<i64>>,
}

struct OrderingRules {
    reverse: HashMap<i64, Vec<i64>>,
}

impl OrderingRules {
    fn new(pairs: impl Iterator<Item = (i64, i64)>) -> Self {
        let mut reverse: HashMap<i64, Vec<i64>> = HashMap::new();

        for (x, y) in pairs {
            reverse.entry(y).or_default().push(x);
        }

        Self { reverse }
    }

    fn is_ok(&self, line: &[i64]) -> bool {
        line.iter().enumerate().all(|(index, nom)| {
            let Some(no_no_noms) = self.reverse.get(nom) else {
                // no requirements for the nom
                return true;
            };

            line.iter().skip(index + 1).all(|n| !no_no_noms.contains(n))
        })
    }

    fn fix_line(&self, line: &mut [i64]) {
        loop {
            let switch = line.iter().enumerate().find_map(|(index, nom)| {
                let Some(no_no_noms) = self.reverse.get(nom) else {
                    // no requirements for the nom
                    return None;
                };

                line.iter()
                    .enumerate()
                    .skip(index + 1)
                    .find(|(_, n)| no_no_noms.contains(n))
                    .map(|(i, _)| (index, i))
            });

            if let Some((x, y)) = switch {
                line[x..(y + 1)].rotate_left(1);
            } else {
                break;
            }
        }
    }
}

impl Update {
    fn from_str(input: &str) -> Self {
        let (first, second) = input.split_once("\n\n").unwrap();
        let ordering_rules = first
            .lines()
            .map(|line| line.split_once('|').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()));
        let pages = second
            .lines()
            .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
            .collect();

        Self {
            ordering_rules: OrderingRules::new(ordering_rules),
            pages,
        }
    }

    fn count_mids(&self) -> i64 {
        let mut count = 0;

        for line in self.pages.iter().filter(|line| self.ordering_rules.is_ok(line)) {
            count += line[line.len() / 2];
        }

        count
    }

    fn count_fixed(&mut self) -> i64 {
        let mut count = 0;

        for line in self.pages.iter_mut().filter(|line| !self.ordering_rules.is_ok(line)) {
            self.ordering_rules.fix_line(line);

            count += line[line.len() / 2];
        }

        count
    }
}

pub fn part1() {
    let update = Update::from_str(INPUT);

    dbg!(update.count_mids());
}

pub fn part2() {
    let mut update = Update::from_str(INPUT);

    dbg!(update.count_fixed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let update = Update::from_str(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );

        assert_eq!(update.count_mids(), 143);
    }

    #[test]
    fn example_two() {
        let mut update = Update::from_str(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );

        assert_eq!(update.count_fixed(), 123);
    }
}
