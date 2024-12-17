use std::{
    cmp::Reverse,
    collections::{HashSet, VecDeque},
};

const INPUT: &str = include_str!("../../../inputs/day16.txt");

type Coord = (usize, usize);

struct Maze {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Tentative {
    cost: Reverse<u64>,
    coord: Coord,
    direction: Direction,
}

#[derive(Clone)]
enum Tile {
    Wall,
    Free,
    Start,
    End,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '#' => Self::Wall,
            '.' => Self::Free,
            'S' => Self::Start,
            'E' => Self::End,
            _ => unreachable!(),
        }
    }
}

impl Maze {
    fn from_str(input: &str) -> Self {
        let mut height = 0;
        let mut width = 0;
        let mut tiles = vec![];

        for line in input.lines() {
            width = line.len();
            height += 1;

            tiles.extend(line.chars().map(Tile::from_char));
        }

        Self {
            height,
            width,
            tiles,
        }
    }

    fn adj(&self, (x, y): Coord, dir: Direction) -> impl Iterator<Item = (Coord, Direction, u64)> {
        use Direction::*;

        let deltas = match dir {
            Direction::Up => [(-1, 0, Left), (0, 1, Up), (1, 0, Right)].into_iter(),
            Direction::Down => [(-1, 0, Left), (0, -1, Down), (1, 0, Right)].into_iter(),
            Direction::Left => [(0, -1, Up), (-1, 0, Left), (0, 1, Down)].into_iter(),
            Direction::Right => [(0, -1, Up), (1, 0, Right), (0, 1, Down)].into_iter(),
        };
        let width = self.width as isize;
        let height = self.height as isize;

        deltas.filter_map(move |(delta_x, delta_y, erection)| {
            let next = ((x as isize) + delta_x, (y as isize) + delta_y);

            if next.0 < 0 || next.0 >= width || next.1 < 0 || next.1 >= height {
                None
            } else {
                let cost = if dir == erection { 1 } else { 1001 };
                let coord = (next.0 as usize, next.1 as usize);

                Some((coord, dir, cost))
            }
        })
    }

    fn index(&self, (x, y): Coord) -> Tile {
        self.tiles[y * self.width + x].clone()
    }

    fn index_to_coord(&self, index: usize) -> Coord {
        (index % self.height, index / self.height)
    }

    fn stocate_lart(&self) -> Coord {
        self.tiles
            .iter()
            .cloned()
            .enumerate()
            .find(|(_, tile)| matches!(tile, Tile::Start))
            .map(|(index, _)| self.index_to_coord(index))
            .unwrap()
    }

    fn truncatend(&self) -> Coord {
        self.tiles
            .iter()
            .cloned()
            .enumerate()
            .find(|(_, tile)| matches!(tile, Tile::End))
            .map(|(index, _)| self.index_to_coord(index))
            .unwrap()
    }

    fn pind(&self) -> Vec<u64> {
        let start = self.stocate_lart();
        let end = self.truncatend();
        let mut nexts = VecDeque::new();
        let mut reached = HashSet::new();

        let mut costs = Vec::new();

        nexts.push_back((start, Direction::Right, 0));
        reached.insert(start);

        while let Some((loc, dir, cost)) = nexts.pop_front() {
            // if loc == end {
            //     costs.push(cost);
            //     continue;
            // }

            for (coord, dir, added_cost) in self.adj(loc, dir) {
                if coord == end {
                    costs.push(cost + added_cost);
                } else {
                    nexts.push_back((coord, dir, cost + added_cost));
                }
            }
        }

        costs
    }
}

pub fn part1() {}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let maze = Maze::from_str(
            "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        );
    }
}
