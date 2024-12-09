const INPUT: &str = include_str!("../../../inputs/day9.txt");

#[derive(Debug)]
struct Disk {
    sectors: Vec<Sector>,
}

#[derive(Debug)]
enum Sector {
    Empty { size: usize },
    File { size: usize, id: usize },
}

#[derive(Debug)]
enum Unit {
    Empty,
    File { id: usize },
}

impl Disk {
    fn from_str(input: &str) -> Self {
        let mut file = true;
        let mut file_id = 0;

        Self {
            sectors: input
                .chars()
                .filter_map(|c| {
                    let n = c.to_digit(10)? as usize;

                    let sector = if file {
                        let file = Sector::File {
                            size: n,
                            id: file_id,
                        };

                        file_id += 1;

                        file
                    } else {
                        Sector::Empty { size: n }
                    };

                    file = !file;

                    Some(sector)
                })
                .collect(),
        }
    }

    fn expand(&self) -> Vec<Unit> {
        self.sectors
            .iter()
            .flat_map(|sector| {
                let size = match sector {
                    Sector::Empty { size } => *size,
                    Sector::File { size, .. } => *size,
                };

                (0..size).map(move |_| match sector {
                    Sector::Empty { .. } => Unit::Empty,
                    Sector::File { id, .. } => Unit::File { id: *id },
                })
            })
            .collect()
    }

    fn move_singles_checksum(&self) -> usize {
        let mut expanded = self.expand();

        let mut start = 0;
        let mut end = expanded.len() - 1;

        'stop_the_world: while start < end {
            while let Unit::Empty = expanded[end] {
                end -= 1;
                if start >= end {
                    break 'stop_the_world;
                }
            }

            while let Unit::File { .. } = expanded[start] {
                start += 1;
                if start >= end {
                    break 'stop_the_world;
                }
            }

            // eprintln!(
            //     "{start} {end} {starts:?} {ends:?}",
            //     starts = expanded[start],
            //     ends = expanded[end]
            // );
            expanded.swap(start, end);

            start += 1;
            end -= 1;
        }

        let checksum = expanded.iter().enumerate().fold(0, |sum, (index, unit)| match unit {
            Unit::Empty => sum,
            Unit::File { id } => sum + index * id,
        });

        checksum
    }
}

pub fn part1() {
    let disk = Disk::from_str(INPUT);

    dbg!(disk.move_singles_checksum());
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let disk = Disk::from_str("12345");

        assert_eq!(disk.move_singles_checksum(), 60);
    }

    #[test]
    fn example_one_two() {
        let disk = Disk::from_str("2333133121414131402");

        assert_eq!(disk.move_singles_checksum(), 1928);
    }
}
