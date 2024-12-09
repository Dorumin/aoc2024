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

    fn reverse_map(&self) -> HashMap<Antenna, HashSet<Coord>> {
        let mut reverse = HashMap::new();
        for (cell, antenna) in self.signalis.iter() {
            reverse
                .entry(antenna.clone())
                .or_insert_with(HashSet::new)
                .insert(*cell);
        }

        reverse
    }

    fn fill_resonances(&mut self) {
        let reverse = self.reverse_map();

        for (antenna, cells) in reverse.into_iter() {
            self.resonances_for(&cells).for_each(|mut resonances| {
                // skip root for 1st in-bounds resonance
                let resonance = resonances.nth(1);

                if let Some(resonance) = resonance {
                    self.resonance.entry(resonance).or_default().insert(antenna.clone());
                }
            });
        }
    }

    fn fill_resonances_repeating(&mut self) {
        let reverse = self.reverse_map();

        for (antenna, cells) in reverse.into_iter() {
            self.resonances_for(&cells).for_each(|resonances| {
                resonances.for_each(|resonance| {
                    self.resonance.entry(resonance).or_default().insert(antenna.clone());
                });
            });
        }
    }

    fn resonances_for<'a>(
        &self,
        coords: &'a HashSet<Coord>,
    ) -> impl Iterator<Item = impl Iterator<Item = Coord> + 'a> + 'a {
        let width = self.width as i64;
        let height = self.height as i64;

        coords.iter().flat_map(move |cell| {
            coords.iter().filter(move |&c| c != cell).map(move |other_cell| {
                let icell = (cell.0 as i64, cell.1 as i64);
                let iother_cell = (other_cell.0 as i64, other_cell.1 as i64);

                let diff = (icell.0 - iother_cell.0, icell.1 - iother_cell.1);
                let mut end = icell;

                (0..).map_while(move |_| {
                    if end.0 < 0 || end.0 >= width || end.1 < 0 || end.1 >= height {
                        // out of bounds
                        return None;
                    }

                    let uend = (end.0 as usize, end.1 as usize);
                    end = (end.0 + diff.0, end.1 + diff.1);

                    Some(uend)
                })
            })
        })
    }
}

pub fn part1() {
    let mut map = City::from_str(INPUT);
    map.fill_resonances();

    dbg!(map.resonance.len());
}

pub fn part2() {
    let mut map = City::from_str(INPUT);
    map.fill_resonances_repeating();

    dbg!(map.resonance.len());
}

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

    #[test]
    fn example_two() {
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

        city.fill_resonances_repeating();

        assert_eq!(city.resonance.len(), 34);
    }
}
