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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Free,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Bot,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Free,
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Bot,
            '[' => Self::BoxLeft,
            ']' => Self::BoxRight,
            _ => unreachable!(),
        }
    }

    fn from_char_fat(c: char) -> (Self, Self) {
        match c {
            '.' => (Self::Free, Self::Free),
            '#' => (Self::Wall, Self::Wall),
            'O' => (Self::BoxLeft, Self::BoxRight),
            '@' => (Self::Bot, Self::Free),
            _ => unreachable!(),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::Free => '.',
            Tile::Wall => '#',
            Tile::Box => 'O',
            Tile::Bot => '@',
            Tile::BoxLeft => '[',
            Tile::BoxRight => ']',
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

    fn from_lines_fat(lines: &mut Lines) -> Self {
        let mut width = 0;
        let mut height = 0;
        let mut tiles = vec![];

        for line in lines {
            if line.is_empty() {
                break;
            }

            height += 1;
            width = line.len() * 2;

            line.chars().for_each(|c| {
                let (left, right) = Tile::from_char_fat(c);
                tiles.push(left);
                tiles.push(right);
            });
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
        (index % self.width, index / self.width)
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

    fn interactive(&mut self, moveset: &Moveset) {
        for (index, mov) in moveset.moves.iter().enumerate() {
            let next = &moveset
                .moves
                .get(index + 1)
                .map(|mv| format!("{mv:?}"))
                .unwrap_or(String::from("none"));

            if index > 950 {
                eprintln!("\u{1b}[2;1H");
                eprintln!("{index} {mov:?} (next: {next})       ");
                self.push(mov.clone());
                eprint!("{self}");

                std::io::stdin().read_line(&mut String::new()).unwrap();
            }
        }
    }

    fn play(&mut self) {
        eprintln!("\u{1b}[2;1H");
        eprint!("{self}");

        loop {
            let mov = loop {
                let c = console::Term::stdout().read_key().unwrap();

                match c {
                    console::Key::ArrowLeft => break Move::Left,
                    console::Key::ArrowRight => break Move::Right,
                    console::Key::ArrowUp => break Move::Up,
                    console::Key::ArrowDown => break Move::Down,
                    console::Key::Escape | console::Key::CtrlC => return,
                    _ => {}
                }
            };

            eprintln!("\u{1b}[2;1H");
            self.push(mov.clone());
            eprint!("{self}");
        }
    }

    fn push(&mut self, mv: Move) {
        let delta: (isize, isize) = match mv {
            Move::Up => (0, -1),
            Move::Down => (0, 1),
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
        };

        let root = self.find_my_robot();

        self.shift_to(root, delta);
    }

    fn shift_to(&mut self, from: Coord, delta: (isize, isize)) {
        let mut patch_list = vec![(from, Tile::Free)];
        let mut cleanup = vec![];
        // "current tiles" which will be checked for free space in front
        let mut current_list = vec![from];

        loop {
            let mut patches = vec![];

            for current in current_list {
                let current_tile = self.index(current);

                let mut next = vec![];

                if !matches!(current_tile, Tile::Free) {
                    cleanup.push(current);

                    next.push((
                        (current.0 as isize + delta.0).try_into().ok().unwrap(),
                        (current.1 as isize + delta.1).try_into().ok().unwrap(),
                        current_tile.clone(),
                    ));
                }

                match current_tile {
                    Tile::BoxLeft if delta.1 != 0 => {
                        cleanup.push((current.0 + 1, current.1));
                        next.push((
                            (current.0 as isize + delta.0 + 1).try_into().ok().unwrap(),
                            (current.1 as isize + delta.1).try_into().ok().unwrap(),
                            Tile::BoxRight,
                        ));
                    }
                    Tile::BoxRight if delta.1 != 0 => {
                        cleanup.push((current.0 - 1, current.1));
                        next.push((
                            (current.0 as isize + delta.0 - 1).try_into().ok().unwrap(),
                            (current.1 as isize + delta.1).try_into().ok().unwrap(),
                            Tile::BoxLeft,
                        ))
                    }
                    _ => {}
                }

                if next.iter().any(|(x, y, _)| *x >= self.width || *y >= self.height) {
                    panic!("out of bounds lmao add walls to the edges you tool");
                }

                if next
                    .iter()
                    .any(|(x, y, _)| matches!(self.index((*x, *y)), Tile::Wall))
                {
                    // eprintln!("walled");
                    return;
                }

                patches.extend(next);
            }

            current_list = patches.iter().map(|(x, y, _)| (*x, *y)).collect();

            patch_list.extend(patches.into_iter().map(|(x, y, tile)| ((x, y), tile)));

            let all_tiles_free = current_list
                .iter()
                .map(|check| self.index(*check))
                .all(|check_tile| matches!(check_tile, Tile::Free));

            if all_tiles_free {
                break;
            }
        }

        for coord in cleanup {
            *self.index_mut(coord) = Tile::Free;
        }

        for (coord, tile) in patch_list {
            *self.index_mut(coord) = tile;
        }
    }

    fn sum(&self) -> u64 {
        self.tiles.iter().enumerate().fold(0, |sum, (index, tile)| {
            if let Tile::Box | Tile::BoxLeft = tile {
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

pub fn part2() {
    let mut lines = INPUT.lines();

    let mut sokoban = Sokoban::from_lines_fat(&mut lines);
    let moveset = Moveset::from_lines(&mut lines);

    if std::env::args().any(|arg| arg == "--interactive") {
        sokoban.interactive(&moveset);
    } else if std::env::args().any(|arg| arg == "--play") {
        sokoban.play();
    } else {
        sokoban.poosh(&moveset);
    }

    dbg!(sokoban.sum());
}

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
        assert_eq!(sokoban.sum(), 2028);
    }

    #[test]
    fn example_fat() {
        let mut lines = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^"
            .lines();

        let mut sokoban = Sokoban::from_lines_fat(&mut lines);
        let moveset = Moveset::from_lines(&mut lines);

        sokoban.poosh(&moveset);

        println!("{sokoban}");
        assert_eq!(sokoban.sum(), 618);
    }

    #[test]
    fn example_big_fat() {
        let mut lines = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            .lines();

        let mut sokoban = Sokoban::from_lines_fat(&mut lines);
        let moveset = Moveset::from_lines(&mut lines);

        sokoban.poosh(&moveset);

        println!("{sokoban}");
        assert_eq!(sokoban.sum(), 9021);
    }

    #[test]
    fn push_down() {
        let mut lines = "\
############
##....[][]##
##..@..[].##
##..[][][]##
##...[][].##
##[]......##
##[]..[]..##
##.[].[][]##
##........##
############

^vvvv"
            .lines();

        let mut sokoban = Sokoban::from_lines(&mut lines);
        let moveset = Moveset::from_lines(&mut lines);

        sokoban.interactive(&moveset);

        assert_eq!(sokoban.sum(), 5978);
    }
}
