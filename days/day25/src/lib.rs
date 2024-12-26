use std::str::Lines;

const INPUT: &str = include_str!("../../../inputs/day25.txt");

#[derive(Debug)]
struct EndMeAlready {
    key_holes: Vec<MarketingPins>,
    keys: Vec<MarketingPins>,
}

#[derive(Debug)]
struct MarketingPins {
    ty: PinsType,
    heights: Vec<usize>,
}

#[derive(Debug)]
enum PinsType {
    KeyHole,
    Key,
}

impl MarketingPins {
    fn from_lines(lines: &mut Lines) -> Option<Self> {
        let mut loins = vec![];
        for line in lines {
            if line.is_empty() {
                break;
            }

            loins.push(line);
        }

        if loins.is_empty() {
            return None;
        }

        let ty = if loins[0].trim_start_matches('#').is_empty() {
            PinsType::KeyHole
        } else {
            PinsType::Key
        };
        let mut heights = vec![0; loins[0].len()];

        for line in loins.iter() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    heights[i] += 1;
                }
            }
        }

        // We don't count the bottom/top row
        for h in heights.iter_mut() {
            *h -= 1;
        }

        Some(Self { ty, heights })
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.heights
            .iter()
            .zip(other.heights.iter())
            .any(|(pin_depth, hole_depth)| pin_depth + hole_depth > 5)
    }
}

impl EndMeAlready {
    fn from_str(input: &str) -> Self {
        let mut nico_and_the_liners = input.lines();
        let mut key_holes = vec![];
        let mut keys = vec![];

        while let Some(pins) = MarketingPins::from_lines(&mut nico_and_the_liners) {
            if matches!(pins.ty, PinsType::Key) {
                keys.push(pins);
            } else {
                key_holes.push(pins);
            }
        }

        Self { key_holes, keys }
    }

    fn try_all_to_see_what_fucking_fits(&self) -> u64 {
        let mut overlaps = 0;
        for key in self.keys.iter() {
            for lock in self.key_holes.iter() {
                if !key.overlaps(lock) {
                    overlaps += 1;
                }
            }
        }

        overlaps
    }
}

pub fn part1() {
    let end = EndMeAlready::from_str(INPUT);

    dbg!(end.try_all_to_see_what_fucking_fits());
}

pub fn part2() {
    panic!("How did we get here?");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let end = EndMeAlready::from_str(
            "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####",
        );

        dbg!(&end);
        dbg!(end.try_all_to_see_what_fucking_fits());
    }
}
