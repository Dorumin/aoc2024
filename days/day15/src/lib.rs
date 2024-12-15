use std::{fmt::Display, str::Lines};

const INPUT: &str = include_str!("../../../inputs/day15.txt");

type Coord = (usize, usize);

#[derive(Debug)]
struct Sokoban {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

#[derive(Debug)]
struct Moveset {
    moves: Vec<Move>,
}

#[derive(Debug, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Tile {
    Free,
    Wall,
    Box,
    Bot,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Free,
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Bot,
            _ => unreachable!(),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::Free => '.',
            Tile::Wall => '#',
            Tile::Box => 'O',
            Tile::Bot => '@',
        }
    }
}

impl Move {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!(),
        }
    }
}

impl Sokoban {
    fn from_lines(lines: &mut Lines) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut tiles = vec![];

        for line in lines {
            if line.is_empty() {
                break;
            }

            height += 1;
            width = line.len();

            line.chars().for_each(|c| tiles.push(Tile::from_char(c)));
        }

        Self {
            width,
            height,
            tiles,
        }
    }

    fn index(&self, (x, y): Coord) -> Tile {
        self.tiles[y * self.width + x].clone()
    }

    fn index_mut(&mut self, (x, y): Coord) -> &mut Tile {
        &mut self.tiles[y * self.width + x]
    }

    fn index_to_coord(&self, index: usize) -> Coord {
        (index % self.height, index / self.height)
    }

    fn find_my_robot(&self) -> Coord {
        self.tiles
            .iter()
            .cloned()
            .enumerate()
            .find(|(_, tile)| matches!(tile, Tile::Bot))
            .map(|(index, _)| self.index_to_coord(index))
            .unwrap()
    }

    fn poosh(&mut self, moveset: &Moveset) {
        moveset.moves.iter().cloned().for_each(|mov| self.push(mov));
    }

    fn push(&mut self, mv: Move) {
        let delta: (isize, isize) = match mv {
            Move::Up => (0, -1),
            Move::Down => (0, 1),
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
        };

        let next_free = |root: (usize, usize), delta: (isize, isize)| {
            let mut check = root;

            loop {
                check.0 = (check.0 as isize + delta.0).try_into().ok()?;
                check.1 = (check.1 as isize + delta.1).try_into().ok()?;

                if check.0 >= self.width || check.1 >= self.height {
                    break None;
                }

                let check_tile = self.index(check);

                if matches!(check_tile, Tile::Wall) {
                    break None;
                }

                if matches!(check_tile, Tile::Free) {
                    break Some(check);
                }
            }
        };

        let root = self.find_my_robot();
        let empty = next_free(root, delta);

        if let Some(free) = empty {
            self.shift_to(root, free, delta);
        }
    }

    fn shift_to(&mut self, from: Coord, to: Coord, delta: (isize, isize)) {
        assert!(matches!(self.index(to), Tile::Free));

        let mut current = to;

        while current != from {
            let prev: Coord = (
                (current.0 as isize - delta.0).try_into().unwrap(),
                (current.1 as isize - delta.1).try_into().unwrap(),
            );

            *self.index_mut(current) = self.index(prev);

            current = prev;
        }

        *self.index_mut(from) = Tile::Free;
    }

    fn sum(&self) -> u64 {
        self.tiles.iter().enumerate().fold(0, |sum, (index, tile)| {
            if let Tile::Box = tile {
                let pos = self.index_to_coord(index);

                sum + pos.1 as u64 * 100 + pos.0 as u64
            } else {
                sum
            }
        })
    }
}

impl Display for Sokoban {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = String::with_capacity(self.width * self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                grid.push(self.index((x, y)).to_char());
            }

            grid.push('\n');
        }

        f.write_str(&grid)
    }
}

impl Moveset {
    fn from_lines(lines: &mut Lines) -> Self {
        let moves = lines.flat_map(|line| line.chars().map(Move::from_char)).collect();

        Self { moves }
    }
}

pub fn part1() {
    let mut lines = INPUT.lines();
    let mut sokoban = Sokoban::from_lines(&mut lines);
    let moveset = Moveset::from_lines(&mut lines);

    sokoban.poosh(&moveset);

    dbg!(sokoban.sum());
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_small() {
        let mut lines = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
            .lines();

        let mut sokoban = Sokoban::from_lines(&mut lines);
        let moveset = Moveset::from_lines(&mut lines);

        sokoban.poosh(&moveset);

        println!("{sokoban}");
    }
}
