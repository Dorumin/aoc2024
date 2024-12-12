const INPUT: &str = include_str!("../../../inputs/day12.txt");

type Coord = (usize, usize);

#[derive(Debug)]
struct Farm {
    tiles: Vec<Vec<Tile>>,
    regions: Vec<Region>,
}

#[derive(Debug, Clone, PartialEq)]
struct Tile(u8);

#[derive(Debug)]
struct Region {
    tile: Tile,
    area: u64,
    perimeter: u64,
    coords: Vec<Coord>,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            'A'..='Z' => Tile(c as u8),
            _ => unreachable!(),
        }
    }
}

impl Farm {
    fn from_str(input: &str) -> Self {
        let mut tiles: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.chars().map(|c| (Tile::from_char(c), false)).collect())
            .collect();

        let regions = Self::carve_regions(&mut tiles);

        let tiles = tiles
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|(tile, walked)| {
                        assert!(walked);
                        tile
                    })
                    .collect()
            })
            .collect();

        Self { tiles, regions }
    }

    fn carve_regions(tiles: &mut [Vec<(Tile, bool)>]) -> Vec<Region> {
        let mut regions = Vec::new();

        for y in 0..tiles.len() {
            for x in 0..tiles[0].len() {
                if tiles[y][x].1 {
                    continue;
                }

                regions.push(Region::carve_out(tiles, (x, y)));
            }
        }

        regions
    }

    fn price(&self) -> u64 {
        self.regions.iter().map(|region| region.price()).sum()
    }
}

impl Region {
    fn carve_out(tiles: &mut [Vec<(Tile, bool)>], (x, y): Coord) -> Self {
        let width = tiles[0].len() as isize;
        let height = tiles.len() as isize;

        let adjacents = |x, y| {
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
        };

        assert!(!tiles[y][x].1);
        tiles[y][x].1 = true;

        let tile = tiles[y][x].0.clone();

        let mut coords = vec![(x, y)];
        let mut processing: Vec<_> = adjacents(x, y).collect();

        while let Some((x, y)) = processing.pop() {
            let (next_tile, walked) = &mut tiles[y][x];

            if *next_tile != tile || *walked {
                continue;
            }
            *walked = true;

            coords.push((x, y));

            processing.extend(adjacents(x, y));
        }

        let area = coords.len() as u64;
        let perimeter = Self::calc_perimeter(&coords, tiles);

        Self {
            tile,
            area,
            perimeter,
            coords,
        }
    }

    fn calc_perimeter(coords: &[Coord], tiles: &[Vec<(Tile, bool)>]) -> u64 {
        let width = tiles[0].len() as isize;
        let height = tiles.len() as isize;

        coords
            .iter()
            .cloned()
            .map(|(x, y)| {
                [(1, 0), (-1, 0), (0, 1), (0, -1)]
                    .into_iter()
                    .filter(move |(delta_x, delta_y)| {
                        let (nx, ny) = ((x as isize) + delta_x, (y as isize) + delta_y);

                        #[allow(clippy::if_same_then_else)]
                        if nx < 0 || nx >= width || ny < 0 || ny >= height {
                            true // OOB is a wall
                        } else if tiles[ny as usize][nx as usize] != tiles[y][x] {
                            true // Build a wall between those that differ from us
                        } else {
                            false
                        }
                    })
                    .count() as u64
            })
            .sum()
    }

    fn price(&self) -> u64 {
        self.area * self.perimeter
    }
}

pub fn part1() {
    let farm = Farm::from_str(INPUT);

    dbg!(farm.price());
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let farm = Farm::from_str(
            "AAAA
BBCD
BBCC
EEEC",
        );

        assert_eq!(farm.regions.len(), 5);

        assert_eq!(farm.price(), 140);

        assert_eq!(farm.regions[0].tile, Tile::from_char('A'));
        assert_eq!(farm.regions[0].area, 4);
        assert_eq!(farm.regions[0].perimeter, 10);

        assert_eq!(farm.regions[1].tile, Tile::from_char('B'));
        assert_eq!(farm.regions[1].area, 4);
        assert_eq!(farm.regions[1].perimeter, 8);

        assert_eq!(farm.regions[2].tile, Tile::from_char('C'));
        assert_eq!(farm.regions[2].area, 4);
        assert_eq!(farm.regions[2].perimeter, 10);

        assert_eq!(farm.regions[3].tile, Tile::from_char('D'));
        assert_eq!(farm.regions[3].area, 1);
        assert_eq!(farm.regions[3].perimeter, 4);

        assert_eq!(farm.regions[4].tile, Tile::from_char('E'));
        assert_eq!(farm.regions[4].area, 3);
        assert_eq!(farm.regions[4].perimeter, 8);
    }
}
