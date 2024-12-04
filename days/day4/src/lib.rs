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
}

pub fn part1() {
    let map = Lettermap::from_str(INPUT);

    dbg!(map.count_xmas());
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
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
}
