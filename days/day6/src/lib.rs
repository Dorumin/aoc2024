use std::collections::HashSet;

const INPUT: &str = include_str!("../../../inputs/day6.txt");

struct Map {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

#[derive(Clone, PartialEq, Debug)]
enum Cell {
    FreeSpace,
    Barrier,
    Playa,
    Walked,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn flag(&self) -> u8 {
        match self {
            Direction::Up => 1,
            Direction::Right => 2,
            Direction::Down => 4,
            Direction::Left => 8,
        }
    }

    fn turn_cockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::FreeSpace,
            '#' => Self::Barrier,
            '^' => Self::Playa,
            _ => unreachable!("pls no"),
        }
    }
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut cells = vec![];
        let mut width = 0;
        let height = input
            .lines()
            .map(|line| {
                width = line.len();

                line.chars().for_each(|c| {
                    if c.is_ascii() {
                        cells.push(Cell::from_char(c));
                    } else {
                        panic!("take yo utf-8 back to python we ascii bytes in this muthafucka")
                    }
                });
            })
            .count();

        Self {
            width,
            height,
            cells,
        }
    }

    fn coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.width).flat_map(|x| (0..self.height).map(move |y| (x, y)))
    }

    fn where_me(&self) -> (usize, usize) {
        self.coords()
            .find(|(x, y)| self.index(*x, *y) == Cell::Playa)
            .unwrap()
    }

    fn index(&self, x: usize, y: usize) -> Cell {
        self.cells[x + y * self.width].clone()
    }

    fn index_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[x + y * self.width]
    }

    fn next_nobarrier(&self, pos: (usize, usize), direction: &Direction) -> Option<(usize, usize)> {
        let next = match direction {
            Direction::Up => (pos.0, pos.1.checked_sub(1)?),
            Direction::Right => (pos.0.checked_add(1)?, pos.1),
            Direction::Down => (pos.0, pos.1.checked_add(1)?),
            Direction::Left => (pos.0.checked_sub(1)?, pos.1),
        };

        if next.0 >= self.width || next.1 >= self.height {
            return None;
        }

        Some(next)
    }

    fn next(
        &self,
        pos: (usize, usize),
        direction: &Direction,
    ) -> Option<((usize, usize), Direction)> {
        let next = self.next_nobarrier(pos, direction)?;

        if self.index(next.0, next.1) == Cell::Barrier {
            Some((pos, direction.turn_cockwise()))
        } else {
            Some((next, direction.clone()))
        }
    }

    fn walk(&mut self) {
        let mut direction = Direction::Up;
        let mut pos = self.where_me();

        while let Some((next, dir)) = self.next(pos, &direction) {
            *self.index_mut(pos.0, pos.1) = Cell::Walked;
            direction = dir;
            pos = next;
        }

        *self.index_mut(pos.0, pos.1) = Cell::Walked;
    }

    fn walked_cells(&self) -> usize {
        self.cells.iter().filter(|&c| *c == Cell::Walked).count()
    }

    fn walk_twisting(&mut self) -> usize {
        let start_position = self.where_me();
        let mut direction = Direction::Up;
        let mut pos = start_position;
        let mut positions = vec![];

        while let Some((next, dir)) = self.next(pos, &direction) {
            positions.push(pos);

            direction = dir;
            pos = next;
        }

        positions.push(pos);

        let mut looped = 0;
        let mut tried = HashSet::new();

        for barry in positions.into_iter() {
            // eprintln!("{start_dir:?} {start_pos:?}");

            let mut local_hookup = vec![0u8; self.cells.len()];
            let mut pos = start_position;
            let mut direction = Direction::Up;

            if tried.contains(&barry) {
                continue;
            }

            tried.insert(barry);

            if barry == start_position {
                // println!("space anomaly");
                continue;
            }

            let old = self.index(barry.0, barry.1);

            if old == Cell::Barrier {
                // println!("no dupes");
                continue;
            }

            // eprintln!("{barry:?}");

            *self.index_mut(barry.0, barry.1) = Cell::Barrier;

            // Our first movement will be blocked by the barrier
            // assert_eq!(
            //     (pos, direction.turn_cockwise()),
            //     self.next(pos, &direction).unwrap()
            // );

            // walk the walk with the new barrier and new state
            while let Some((next, dir)) = self.next(pos, &direction) {
                let flog = dir.flag();
                let ind = next.0 + next.1 * self.width;

                if local_hookup[ind] & flog == flog {
                    // println!("found loopy banoopy at {dir:?} {next:?}, barrier at {barry:?}");

                    looped += 1;

                    break;
                } else {
                    // println!("move {pos:?} to {next:?}");
                    local_hookup[ind] |= dir.flag();

                    direction = dir;
                    pos = next;
                }
            }

            *self.index_mut(barry.0, barry.1) = old;
        }

        looped
    }
}

pub fn part1() {
    let mut map = Map::from_str(INPUT);

    map.walk();

    dbg!(map.walked_cells());
}

pub fn part2() {
    let mut map = Map::from_str(INPUT);

    dbg!(map.walk_twisting());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut map = Map::from_str(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );

        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);

        map.walk();

        assert_eq!(map.walked_cells(), 41);
    }

    #[test]
    fn example_two() {
        let mut map = Map::from_str(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );

        assert_eq!(map.walk_twisting(), 6);
    }
}
