#![allow(unused)]

use std::collections::HashSet;

const INPUT: &str = include_str!("../../../inputs/day12.txt");

type Coord = (usize, usize);

#[derive(Debug)]
struct Farm {
    tiles: Vec<Vec<Tile>>,
    regions: Vec<Region>,
}

#[derive(Debug, Clone, PartialEq)]
struct Tile(char);

#[derive(Debug)]
struct Region {
    tile: Tile,
    area: u64,
    perimeter: u64,
    sides: u64,
    coords: Vec<Coord>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Edge(usize, usize, Flow);

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Flow {
    Down,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Down,
    Right,
    Up,
    Left,
}

impl Direction {
    fn from_flow(flow: &Flow) -> Self {
        match flow {
            Flow::Down => Self::Down,
            Flow::Right => Self::Right,
        }
    }

    fn left(&self) -> Self {
        match self {
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
        }
    }
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            'A'..='Z' => Tile(c),
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

    fn price_straight(&self) -> u64 {
        self.regions.iter().map(|region| region.price_straight()).sum()
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
        let (perimeter, sides) = Self::calc_perimeter(&coords, tiles);

        // eprintln!("final sides {sides} for {tile:?}");

        Self {
            tile,
            area,
            perimeter,
            sides,
            coords,
        }
    }

    fn calc_perimeter(coords: &[Coord], tiles: &[Vec<(Tile, bool)>]) -> (u64, u64) {
        let width = tiles[0].len() as isize;
        let height = tiles.len() as isize;

        let mut edges = Vec::new();

        let perimeter = coords
            .iter()
            .cloned()
            .map(|(x, y)| {
                [(1, 0), (-1, 0), (0, 1), (0, -1)]
                    .into_iter()
                    .filter(|(delta_x, delta_y)| {
                        let (nx, ny) = ((x as isize) + delta_x, (y as isize) + delta_y);
                        let flow = if delta_y.abs() == 1 {
                            Flow::Right
                        } else {
                            Flow::Down
                        };

                        // Out of bounds
                        if nx < 0 || ny < 0 {
                            edges.push(Edge(x, y, flow));

                            true
                        } else if nx >= width {
                            edges.push(Edge(x + 1, y, flow));

                            true
                        } else if ny >= height {
                            edges.push(Edge(x, y + 1, flow));

                            true
                        // Build a wall between those that differ from us
                        } else if tiles[ny as usize][nx as usize] != tiles[y][x] {
                            edges.push(Edge(
                                x + (*delta_x).max(0) as usize,
                                y + (*delta_y).max(0) as usize,
                                flow,
                            ));

                            true
                        } else {
                            false
                        }
                    })
                    .count() as u64
            })
            .sum();

        assert_eq!(edges.len() as u64, perimeter);

        let mut edged: HashSet<Edge> = HashSet::new();
        let mut edgeset = HashSet::new();

        edgeset.extend(edges.iter().cloned());

        // reading is not allowed

        edges.sort();

        // dbg!(&edges);

        let mut turns = 0;

        fn left(current: &Edge, direction: &Direction) -> Option<Edge> {
            Some(match direction {
                Direction::Down => Edge(current.0, current.1 + 1, Flow::Right),
                // right to up is unanchored
                Direction::Right => Edge(current.0 + 1, current.1.checked_sub(1)?, Flow::Down),
                // up to left is unanchored
                Direction::Up => Edge(current.0.checked_sub(1)?, current.1, Flow::Right),
                Direction::Left => Edge(current.0, current.1, Flow::Down),
            })
        }

        fn right(current: &Edge, direction: &Direction) -> Option<Edge> {
            Some(match direction {
                // down to left is unanchored
                Direction::Down => Edge(current.0.checked_sub(1)?, current.1 + 1, Flow::Right),
                Direction::Right => Edge(current.0 + 1, current.1, Flow::Down),
                Direction::Up => Edge(current.0, current.1, Flow::Right),
                // left to up is unanchored
                Direction::Left => Edge(current.0, current.1.checked_sub(1)?, Flow::Down),
            })
        }

        fn forward_lmao(current: &Edge, direction: &Direction) -> Option<Edge> {
            Some(match direction {
                Direction::Down => Edge(current.0, current.1 + 1, Flow::Down),
                Direction::Right => Edge(current.0 + 1, current.1, Flow::Right),
                Direction::Up => Edge(current.0, current.1.checked_sub(1)?, Flow::Down),
                Direction::Left => Edge(current.0.checked_sub(1)?, current.1, Flow::Right),
            })
        }

        for edge in edges.iter() {
            if edged.contains(edge) {
                continue;
            }

            let mut direction = Direction::from_flow(&edge.2);
            let mut current = edge.clone();
            edged.insert(current.clone());

            // eprintln!("starting from {current:?}");

            turns += 1;

            loop {
                let next_left = left(&current, &direction);
                let next_right = right(&current, &direction);
                let next_straight = forward_lmao(&current, &direction);

                // dbg!(&next_left);
                // dbg!(&next_right);
                // dbg!(&next_straight);

                if let Some(left) = next_left {
                    if edgeset.contains(&left) && !edged.contains(&left) {
                        direction = direction.left();
                        // eprintln!("turning left {left:?}, going {direction:?}");
                        current = left;
                        edged.insert(current.clone());

                        // if let Some(forward) = next_straight {
                        //     if direction == Direction::Down || direction == Direction::Left {
                        //         eprintln!("!!! marked {forward:?}");
                        //         edged.insert(forward);
                        //     }
                        // }

                        turns += 1;
                        continue;
                    }
                }

                if let Some(right) = next_right {
                    if edgeset.contains(&right) && !edged.contains(&right) {
                        direction = direction.right();
                        // eprintln!("turning right {right:?}, going {direction:?}");
                        current = right;
                        edged.insert(current.clone());

                        // if let Some(forward) = next_straight {
                        //     if direction == Direction::Up || direction == Direction::Right {
                        //         eprintln!("!!! marked {forward:?}");
                        //         edged.insert(forward);
                        //     }
                        // }

                        turns += 1;

                        continue;
                    }
                }

                if let Some(forward) = next_straight {
                    if edged.contains(&forward) {
                        break;
                    }

                    if edgeset.contains(&forward) {
                        // eprintln!("forward {forward:?}");
                        current = forward;
                        edged.insert(current.clone());

                        continue;
                    }
                }

                // dbg!(left(&current, &direction));
                // dbg!(right(&current, &direction));
                // dbg!(forward_lmao(&current, &direction));
                // dbg!(current, direction);

                // unreachable!("should never get here");
                // eprintln!("sides so far: {turns}");
                break;
            }
        }

        // let sides = edges
        //     .iter()
        //     .filter(|&edge| {
        //         let was_edged = edged.contains(edge);

        //         let dx = (edge.2 == Flow::Down) as usize;
        //         let dy = (edge.2 == Flow::Right) as usize;

        //         let next = edge.next();
        //         let next_right = Edge(next.0 + dx, next.1 + dy, next.2.rev());
        //         let next_left = if let Some(x) = next.0.checked_sub(dx) {
        //             if let Some(y) = next.1.checked_sub(dy) {
        //                 Some(Edge(x, y, next.2.rev()))
        //             } else {
        //                 None
        //             }
        //         } else {
        //             None
        //         };

        //         if !edges.contains(&next_right) && !next_left.is_some_and(|v| edges.contains(&v)) {
        //             edged.insert(next);
        //         }
        //         if let Some(prev) = edge.prev() {
        //             edged.insert(prev);
        //         }

        //         !was_edged
        //     })
        //     .count() as u64;

        (perimeter, turns)
    }

    fn price(&self) -> u64 {
        self.area * self.perimeter
    }

    fn price_straight(&self) -> u64 {
        self.area * self.sides
    }
}

pub fn part1() {
    let farm = Farm::from_str(INPUT);

    dbg!(farm.price());
}

pub fn part2() {
    let farm = Farm::from_str(INPUT);

    dbg!(farm.price_straight());
}

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
        assert_eq!(farm.price_straight(), 80);

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

    #[test]
    fn example_two() {
        let farm = Farm::from_str(
            "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
        );

        assert_eq!(farm.price(), 1930);
        assert_eq!(farm.price_straight(), 1206);
    }

    #[test]
    fn example_three() {
        let farm = Farm::from_str(
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        );

        assert_eq!(farm.price_straight(), 368);
    }

    #[test]
    fn example_four() {
        let farm = Farm::from_str(
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        );

        assert_eq!(farm.price_straight(), 236);
    }
}
