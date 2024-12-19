use cached::proc_macro::cached;
use regex::Regex;

const INPUT: &str = include_str!("../../../inputs/day19.txt");

struct Ojisan {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl Ojisan {
    fn from_str(input: &str) -> Self {
        let mut lines = input.lines();
        let patterns = lines.next().unwrap().split(", ").map(|s| s.to_string()).collect();
        assert_eq!(lines.next(), Some(""));

        let designs = lines.map(|line| line.to_string()).collect();

        Self { patterns, designs }
    }

    fn possible_count(&self) -> usize {
        let pat = format!("^({})+$", self.patterns.join("|"));

        let rex = Regex::new(&pat).unwrap();
        self.designs.iter().filter(|des| rex.is_match(des)).count()
    }

    fn possibilities_count(&self) -> usize {
        self.designs.iter().map(|des| self.count_possibilities(des)).sum()
    }

    fn count_possibilities(&self, design: &str) -> usize {
        #[cached(key = "String", convert = " { precinct.to_string() }")]
        fn fp_wins_again(patterns: &[String], precinct: &str) -> usize {
            patterns
                .iter()
                .map(|pat| {
                    if let Some(rest) = precinct.strip_prefix(pat) {
                        if rest.is_empty() {
                            1
                        } else {
                            fp_wins_again(patterns, rest)
                        }
                    } else {
                        0
                    }
                })
                .sum()
        }

        fp_wins_again(&self.patterns, design)
    }
}

pub fn part1() {
    let ojisan = Ojisan::from_str(INPUT);

    dbg!(ojisan.possible_count());
}

pub fn part2() {
    let ojisan = Ojisan::from_str(INPUT);

    dbg!(ojisan.possibilities_count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let ojisan = Ojisan::from_str(
            "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        );

        dbg!(ojisan.possible_count());
        dbg!(ojisan.possibilities_count());
    }
}
