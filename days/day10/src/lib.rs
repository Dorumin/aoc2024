use std::collections::HashSet;

const INPUT: &str = include_str!("../../../inputs/day10.txt");

type Coord = (usize, usize);

// No bottoms allowed
struct TopMap {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Tile(u8);

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '0'..='9' => Self(c.to_digit(10).unwrap() as u8),
            '.' => Self::impassable(),
            _ => unreachable!(),
        }
    }

    fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    fn is_end(&self) -> bool {
        self.0 == 9
    }

    fn impassable() -> Self {
        Self(42)
    }
}

impl TopMap {
    fn from_str(input: &str) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut tiles = Vec::with_capacity(input.len());

        input.lines().for_each(|line| {
            height += 1;
            width = line.len();

            line.chars().for_each(|c| {
                tiles.push(Tile::from_char(c));
            });
        });

        Self {
            width,
            height,
            tiles,
        }
    }

    fn index(&self, (x, y): Coord) -> Tile {
        self.tiles[y * self.width + x]
    }

    fn index_to_coord(&self, index: usize) -> Coord {
        (index % self.height, index / self.height)
    }

    fn starting_tiles(&self) -> impl Iterator<Item = Coord> + '_ {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| **tile == Tile(0))
            .map(|(i, _)| self.index_to_coord(i))
    }

    fn adjacents(&self, (x, y): Coord) -> impl Iterator<Item = Coord> + '_ {
        let width = self.width as isize;
        let height = self.height as isize;

        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .filter_map(move |(delta_x, delta_y)| {
                let next = ((x as isize) + delta_x, (y as isize) + delta_y);

                if next.0 < 0 || next.0 >= width || next.1 < 0 || next.1 >= height {
                    None
                } else {
                    Some((next.0 as usize, next.1 as usize))
                }
            })
    }

    fn adjacents_increasing(
        &self,
        start: Coord,
        value: Tile,
    ) -> impl Iterator<Item = (Tile, Coord)> + '_ {
        self.adjacents(start).filter_map(move |next| {
            let next_value = self.index(next);

            if next_value == value.next() {
                // eprintln!("{start:?} {value:?} {next:?} {next_value:?}");
                Some((next_value, next))
            } else {
                None
            }
        })
    }

    fn score_for(&self, coord: Coord, repeats: bool) -> usize {
        let mut visited: HashSet<Coord> = HashSet::new();
        let mut score = 0;
        let mut next: Vec<_> = self.adjacents_increasing(coord, self.index(coord)).collect();

        // This isn't lisp, take your recursion and parentheses elsewhere
        // It's also not haskell, where are the typeclasses?
        while !next.is_empty() {
            let nexts = next.iter().flat_map(|(tile, coord)| {
                if tile.is_end() {
                    if repeats {
                        score += 1;
                    } else if !visited.contains(coord) {
                        visited.insert(*coord);
                        score += 1;
                    }

                    Box::new(std::iter::empty()) as Box<dyn std::iter::Iterator<Item = _>>
                } else {
                    Box::new(self.adjacents_increasing(*coord, *tile))
                }
            });

            next = nexts.collect();
        }

        score
    }

    fn scores_increasing_total(&self, repeats: bool) -> usize {
        self.starting_tiles()
            .map(|start| self.score_for(start, repeats))
            .sum()
    }
}

pub fn part1() {
    let map = TopMap::from_str(INPUT);

    dbg!(map.scores_increasing_total(false));
}

pub fn part2() {
    let map = TopMap::from_str(INPUT);

    dbg!(map.scores_increasing_total(true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let map = TopMap::from_str(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );

        assert_eq!(map.score_for((2, 0), false), 5);
        assert_eq!(map.scores_increasing_total(false), 36)
    }

    #[test]
    fn example_one_two() {
        let map = TopMap::from_str(
            "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
        );

        assert_eq!(map.scores_increasing_total(false), 2)
    }

    #[test]
    fn example_one_four() {
        let map = TopMap::from_str(
            "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
        );

        assert_eq!(map.scores_increasing_total(false), 4)
    }

    #[test]
    fn example_two() {
        let map = TopMap::from_str(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );

        assert_eq!(map.scores_increasing_total(true), 81)
    }

    #[test]
    fn starts() {
        let map = TopMap::from_str(
            "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01",
        );

        assert_eq!(map.starting_tiles().count(), 2)
    }

    #[test]
    fn coords() {
        let map = TopMap::from_str(INPUT);

        for i in 0..100 {
            let c = map.index_to_coord(i);

            assert_eq!(map.tiles[i], map.index(c));
        }
    }
}
