use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    // also known as "north"
    Up,
    // also known as "south"
    Down,
    // also known as "starboard"
    Left,
    // also known as "starboy"
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
            Direction::Up => [(-1, 0, Left), (0, -1, Up), (1, 0, Right)].into_iter(),
            Direction::Down => [(-1, 0, Left), (0, 1, Down), (1, 0, Right)].into_iter(),
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
                // eprintln!("{cost} {dir:?} {erection:?} {coord:?}");

                Some((coord, erection, cost))
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

    fn pind(&self) -> (u64, Vec<Vec<Coord>>) {
        let start = self.stocate_lart();
        let end = self.truncatend();
        // let mut nexts = VecDeque::new();
        // let mut reached = HashSet::new();

        // let mut costs = Vec::new();

        let mut nexts = BinaryHeap::new();
        // let mut costs = HashMap::new();
        // let mut paths = HashMap::new();

        let mut others = HashMap::new();

        others.insert(start, (0, vec![vec![]]));

        // costs.insert(start, 0);
        // paths.insert(start, vec![vec![start]]);

        nexts.push(Tentative {
            cost: Reverse(0),
            coord: start,
            direction: Direction::Right,
        });

        // nexts.push_back((start, Direction::Right, 0));
        // reached.insert(start);

        while let Some(Tentative {
            cost: Reverse(cost),
            coord,
            direction,
        }) = nexts.pop()
        {
            eprintln!("nexts: {} others {}", nexts.len(), others.len());
            eprintln!("at {coord:?} going {direction:?} for {cost:?}");

            let other = others.get(&coord).cloned();

            if let Some((other_cost, _)) = other.as_ref() {
                // && is too hard for a 10 year old language to figure out
                if *other_cost < cost {
                    continue;
                }
            }

            for (next_coord, direction, added_cost) in self.adj(coord, direction) {
                if matches!(self.index(next_coord), Tile::Wall) {
                    continue;
                }

                let new_cost = cost + added_cost;

                let other_cost = others
                    .get(&next_coord)
                    .as_ref()
                    .map(|other| other.0)
                    .unwrap_or(u64::MAX);

                if new_cost < other_cost {
                    // eprintln!("{coord:?} to {next_coord:?} for {new_cost} over {other_cost}");

                    let mut new_paths = other.as_ref().map(|o| o.1.clone()).unwrap_or_default();

                    new_paths.iter_mut().for_each(|p| p.push(coord));

                    others.insert(next_coord, (new_cost, new_paths));

                    nexts.push(Tentative {
                        cost: Reverse(new_cost),
                        coord: next_coord,
                        direction: direction.clone(),
                    });
                }

                if new_cost == other_cost {
                    eprintln!("new cost is same as other cost");

                    let mut new_paths = other.as_ref().map(|o| o.1.clone()).unwrap_or_default();

                    new_paths.iter_mut().for_each(|p| p.push(coord));

                    let paths = &mut others.get_mut(&next_coord).unwrap().1;

                    for new_path in new_paths {
                        paths.push(new_path);
                    }

                    // nexts.push(Tentative {
                    //     cost: Reverse(new_cost),
                    //     coord: next_coord,
                    //     direction,
                    // });
                }

                // if coord == end {
                //     costs.push(cost + added_cost);
                // } else {
                //     nexts.push_back((coord, dir, cost + added_cost));
                // }
            }
        }

        others.get(&end).unwrap().clone()
    }
}

pub fn part1() {
    let maze = Maze::from_str(INPUT);

    dbg!(maze.pind().0);
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro() {
        let maze = Maze::from_str(
            "\
####
#.E#
#S.#
####",
        );

        assert_eq!(maze.pind().0, 1002);
    }

    #[test]
    fn mini() {
        let maze = Maze::from_str(
            "\
#####
#..E#
#.#.#
#S..#
#####",
        );

        assert_eq!(maze.pind().0, 1004);
    }

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

        assert_eq!(maze.pind().0, 7036);
    }

    #[test]
    fn example_two() {
        let maze = Maze::from_str(
            "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
        );

        dbg!(maze.pind().1.len());
        assert_eq!(maze.pind().0, 11048);
    }
}
