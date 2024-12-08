use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../../inputs/day8.txt");

type Coord = (usize, usize);

#[derive(Debug)]
struct City {
    width: usize,
    height: usize,
    signalis: HashMap<Coord, Antenna>,
    resonance: HashMap<Coord, HashSet<Antenna>>,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct Antenna(u8);

impl Antenna {
    fn from_char(c: char) -> Option<Self> {
        match c {
            // 48..=57 -> 0..=9
            '0'..='9' => Some(Antenna(c as u8 - 48)),
            // 65..=90 -> 10..=35
            'A'..='Z' => Some(Antenna(c as u8 - 55)),
            // 97..=122 -> 36..=61
            'a'..='z' => Some(Antenna(c as u8 - 61)),
            _ => None,
        }
    }
}

impl City {
    fn from_str(input: &str) -> Self {
        let mut signalis = HashMap::new();
        let resonance = HashMap::new();

        let mut width = 0;
        let height = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                width = line.len();

                line.chars().enumerate().for_each(|(x, c)| {
                    if let Some(antenna) = Antenna::from_char(c) {
                        let coord = (x, y);

                        signalis.insert(coord, antenna);
                    }
                });
            })
            .count();

        Self {
            width,
            height,
            signalis,
            resonance,
        }
    }

    fn fill_resonances(&mut self) {
        let mut reverse = HashMap::new();
        for (cell, antenna) in self.signalis.iter() {
            reverse.entry(antenna).or_insert_with(HashSet::new).insert(*cell);
        }

        for (antenna, cells) in reverse.into_iter() {
            for cell in cells.iter() {
                for other_cell in cells.iter().filter(|&c| c != cell) {
                    let icell = (cell.0 as i64, cell.1 as i64);
                    let iother_cell = (other_cell.0 as i64, other_cell.1 as i64);

                    let diff = (icell.0 - iother_cell.0, icell.1 - iother_cell.1);
                    let end = (icell.0 + diff.0, icell.1 + diff.1);

                    if end.0 < 0
                        || end.0 > self.width as i64
                        || end.1 < 0
                        || end.1 > self.height as i64
                    {
                        // out of bounds
                        continue;
                    }

                    let end = (end.0 as usize, end.1 as usize);

                    self.resonance.entry(end).or_default().insert(antenna.clone());
                }
            }
        }
    }
}

pub fn part1() {
    let mut map = City::from_str(INPUT);
    map.fill_resonances();

    dbg!(map.resonance.len());
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut city = City::from_str(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        );

        city.fill_resonances();

        assert_eq!(city.resonance.len(), 14);
    }
}
