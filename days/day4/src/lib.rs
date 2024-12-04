const INPUT: &str = include_str!("../../../inputs/day4.txt");

struct Lettermap {
    chars: Vec<Vec<char>>,
}

impl Lettermap {
    fn from_str(s: &str) -> Self {
        Self {
            chars: s.lines().map(|line| line.chars().collect()).collect(),
        }
    }

    fn col_count(&self) -> usize {
        self.chars[0].len()
    }

    fn match_pattern(&self, pattern: &[&[fn(char) -> bool]]) -> usize {
        let row_count = pattern.len();
        let col_count = pattern[0].len();
        let mut count = 0;

        for row in 0..(self.chars.len() - row_count) {
            for col in 0..(self.col_count() - col_count) {
                let matched = pattern.iter().enumerate().all(|(i, pats)| {
                    pats.iter()
                        .enumerate()
                        .all(|(j, pat)| pat(self.chars[row + i][col + j]))
                });

                if matched {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_xmas(&self) -> usize {
        let mut count = 0;

        for row in 0..self.chars.len() {
            let r = &self.chars[row];

            for col in 0..(self.col_count() - 3) {
                if r[col] == 'X' && r[col + 1] == 'M' && r[col + 2] == 'A' && r[col + 3] == 'S' {
                    count += 1;
                }

                if r[col] == 'S' && r[col + 1] == 'A' && r[col + 2] == 'M' && r[col + 3] == 'X' {
                    count += 1;
                }
            }
        }

        let c = &self.chars;
        for col in 0..self.col_count() {
            for row in 0..(self.chars.len() - 3) {
                if c[row][col] == 'X'
                    && c[row + 1][col] == 'M'
                    && c[row + 2][col] == 'A'
                    && c[row + 3][col] == 'S'
                {
                    count += 1;
                }

                if c[row][col] == 'S'
                    && c[row + 1][col] == 'A'
                    && c[row + 2][col] == 'M'
                    && c[row + 3][col] == 'X'
                {
                    count += 1;
                }
            }
        }

        for col in 0..(self.col_count() - 3) {
            for row in 0..(self.chars.len() - 3) {
                if c[row][col] == 'X'
                    && c[row + 1][col + 1] == 'M'
                    && c[row + 2][col + 2] == 'A'
                    && c[row + 3][col + 3] == 'S'
                {
                    count += 1;
                }

                if c[row][col] == 'S'
                    && c[row + 1][col + 1] == 'A'
                    && c[row + 2][col + 2] == 'M'
                    && c[row + 3][col + 3] == 'X'
                {
                    count += 1;
                }
            }
        }

        for col in 3..self.col_count() {
            for row in 0..(self.chars.len() - 3) {
                if c[row][col] == 'X'
                    && c[row + 1][col - 1] == 'M'
                    && c[row + 2][col - 2] == 'A'
                    && c[row + 3][col - 3] == 'S'
                {
                    count += 1;
                }

                if c[row][col] == 'S'
                    && c[row + 1][col - 1] == 'A'
                    && c[row + 2][col - 2] == 'M'
                    && c[row + 3][col - 3] == 'X'
                {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_x_mas(&self) -> usize {
        let mut count = 0;

        let c = &self.chars;
        for row in 0..(self.chars.len() - 2) {
            for col in 0..(self.col_count() - 2) {
                if c[row + 1][col + 1] == 'A'
                    && ((c[row][col] == 'M' && c[row + 2][col + 2] == 'S')
                        || (c[row][col] == 'S' && c[row + 2][col + 2] == 'M'))
                    && ((c[row][col + 2] == 'M' && c[row + 2][col] == 'S')
                        || (c[row][col + 2] == 'S' && c[row + 2][col] == 'M'))
                {
                    count += 1;
                }
            }
        }

        count
    }

    fn count_x_mas_cooler(&self) -> usize {
        self.match_pattern(&[
            &[|c| c == 'S', |_| true, |c| c == 'M'],
            &[|_| true, |c| c == 'A', |_| true],
            &[|c| c == 'S', |_| true, |c| c == 'M'],
        ]) + self.match_pattern(&[
            &[|c| c == 'M', |_| true, |c| c == 'M'],
            &[|_| true, |c| c == 'A', |_| true],
            &[|c| c == 'S', |_| true, |c| c == 'S'],
        ]) + self.match_pattern(&[
            &[|c| c == 'S', |_| true, |c| c == 'S'],
            &[|_| true, |c| c == 'A', |_| true],
            &[|c| c == 'M', |_| true, |c| c == 'M'],
        ]) + self.match_pattern(&[
            &[|c| c == 'M', |_| true, |c| c == 'S'],
            &[|_| true, |c| c == 'A', |_| true],
            &[|c| c == 'M', |_| true, |c| c == 'S'],
        ])
    }
}

pub fn part1() {
    let map = Lettermap::from_str(INPUT);

    dbg!(map.count_xmas());
}

pub fn part2() {
    let map = Lettermap::from_str(INPUT);

    dbg!(map.count_x_mas());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ejemplo_uno() {
        let map = Lettermap::from_str(
            "....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX",
        );

        assert_eq!(map.count_xmas(), 18);
    }

    #[test]
    fn ejemplo_dos() {
        let map = Lettermap::from_str(
            ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
        );

        assert_eq!(map.count_x_mas(), 9);
    }

    #[test]
    fn ejemplo_dos_cooler() {
        let map = Lettermap::from_str(
            ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
        );

        assert_eq!(map.count_x_mas_cooler(), 9);
    }
}
