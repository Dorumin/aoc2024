const INPUT: &str = include_str!("../../../inputs/day1.txt");

struct Lines {
    left: Vec<i64>,
    right: Vec<i64>,
}

impl Lines {
    fn from_str(input: &str) -> Self {
        let pairs = input.lines().map(|line| {
            let mut split = line.split_whitespace();
            let left: i64 = split.next().expect("no lefty").parse().expect("lefty nan");
            let right: i64 = split
                .next()
                .expect("no righty")
                .parse()
                .expect("righty nan");

            (left, right)
        });

        let (left, right) = pairs.unzip();

        Self { left, right }
    }

    fn sort(&mut self) {
        self.left.sort();
        self.right.sort();
    }
}

impl IntoIterator for Lines {
    type IntoIter = std::iter::Zip<std::vec::IntoIter<i64>, std::vec::IntoIter<i64>>;
    type Item = (i64, i64);

    fn into_iter(self) -> Self::IntoIter {
        self.left.into_iter().zip(self.right)
    }
}

pub fn part1() {
    let mut lines = Lines::from_str(INPUT);
    lines.sort();

    let difference: i64 = lines.into_iter().map(|(a, b)| (a - b).abs()).sum();

    dbg!(difference);
}

pub fn part2() {
    let lines = Lines::from_str(INPUT);

    // Optimizing at this scale is pretty pointless
    let similarities: i64 = lines
        .left
        .iter()
        .map(|a| a * lines.right.iter().filter(|b| a == *b).count() as i64)
        .sum();

    dbg!(similarities);
}
