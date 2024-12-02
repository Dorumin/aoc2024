const INPUT: &str = include_str!("../../../inputs/day2.txt");

struct Reports {
    lines: Vec<Line>,
}

struct Line {
    values: Vec<i64>,
}

impl Reports {
    fn from_str(s: &str) -> Self {
        Self {
            lines: s.lines().map(Line::from_str).collect(),
        }
    }

    fn safe_line_count(&self) -> usize {
        self.lines.iter().filter(|line| line.is_safe()).count()
    }
}

impl Line {
    fn from_str(s: &str) -> Self {
        Self {
            values: s.split_whitespace().map(|n| n.parse().unwrap()).collect(),
        }
    }

    fn is_increasing(&self) -> bool {
        self.values.windows(2).all(|a| a[0] < a[1])
    }

    fn is_decreasing(&self) -> bool {
        self.values.windows(2).all(|a| a[0] > a[1])
    }

    fn is_stable(&self) -> bool {
        self.values.windows(2).all(|a| (a[0] - a[1]).abs() <= 3)
    }

    pub fn is_safe(&self) -> bool {
        (self.is_increasing() || self.is_decreasing()) && self.is_stable()
    }
}

pub fn part1() {
    let safes = Reports::from_str(INPUT).safe_line_count();

    dbg!(safes);
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo() {
        assert!(Line::from_str("7 6 4 2 1").is_safe());
        assert!(!Line::from_str("1 2 7 8 9").is_safe());
        assert!(!Line::from_str("9 7 6 2 1").is_safe());
        assert!(!Line::from_str("1 3 2 4 5").is_safe());
        assert!(!Line::from_str("8 6 4 4 1").is_safe());
        assert!(Line::from_str("1 3 6 7 9").is_safe());
    }
}
