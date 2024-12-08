const INPUT: &str = include_str!("../../../inputs/dayN.txt");

struct Thing {}

impl Thing {
    fn from_str(input: &str) -> Self {
        Self {}
    }
}

pub fn part1() {}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let thing = Thing::from_str("");
    }
}
