const INPUT: &str = include_str!("../../../inputs/day20.txt");

type Coord = (usize, usize);

#[derive(Debug)]
struct Code {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Free,
    Wall,
    Start,
    End,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Tile::Free,
            '#' => Tile::Wall,
            'S' => Tile::Start,
            'E' => Tile::End,
            _ => unreachable!(),
        }
    }
}

impl Code {
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
            width,
            height,
            tiles,
        }
    }

    fn index(&self, (x, y): Coord) -> Tile {
        self.tiles[y * self.width + x]
    }

    fn adj(&self, (x, y): Coord) -> impl Iterator<Item = Coord> {
        let width = self.width as isize;
        let height = self.height as isize;

        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .filter_map(move |(delta_x, delta_y)| {
                let next = ((x as isize) + delta_x, (y as isize) + delta_y);

                if next.0 < 0 || next.0 >= width || next.1 < 0 || next.1 >= height {
                    None
                } else {
                    let coord = (next.0 as usize, next.1 as usize);

                    Some(coord)
                }
            })
    }

    fn index_to_coord(&self, index: usize) -> Coord {
        (index % self.width, index / self.width)
    }

    fn start(&self) -> Coord {
        self.tiles
            .iter()
            .enumerate()
            .find(|(_, tile)| **tile == Tile::Start)
            .map(|(i, _)| self.index_to_coord(i))
            .unwrap()
    }

    fn end(&self) -> Coord {
        self.tiles
            .iter()
            .enumerate()
            .find(|(_, tile)| **tile == Tile::End)
            .map(|(i, _)| self.index_to_coord(i))
            .unwrap()
    }

    fn path_lmao(&self) -> Vec<Coord> {
        let start = self.start();
        let end = self.end();

        let mut coords = vec![];
        let mut current = start;
        let mut previous = start;

        while current != end {
            let next = self
                .adj(current)
                .find(|next| !matches!(self.index(*next), Tile::Wall) && *next != previous)
                .unwrap();

            coords.push(current);
            previous = current;
            current = next;
        }

        coords.push(current);

        for (index, tile) in self.tiles.iter().enumerate() {
            if !matches!(tile, Tile::Wall) {
                let coord = self.index_to_coord(index);

                assert!(coords.contains(&coord));
            }
        }

        coords
    }

    fn hacks_for<'a>(&'a self, path: &'a [Coord]) -> impl Iterator<Item = usize> + 'a {
        path.iter().enumerate().flat_map(move |(index, coord)| {
            let adjacents =
                self.adj(*coord)
                    .flat_map(|adj| self.adj(adj))
                    .filter_map(move |skipped| {
                        path.iter().enumerate().skip(index).find_map(|(cont, c)| {
                            if skipped == *c {
                                Some(index + 2 + (path.len() - cont))
                            } else {
                                None
                            }
                        })
                    });

            adjacents
        })
    }

    // It's not always DP
    fn op_hacks(&self) -> u64 {
        let path = self.path_lmao();

        // Mapping beats a 9k cell scan
        let mut indexes = vec![usize::MAX; self.width * self.height];
        for (index, coord) in path.iter().enumerate() {
            indexes[coord.1 * self.width + coord.0] = index;
        }

        // score is len - 1; to beat is at least 100 less than that
        // even accounting for this +1 from including start pos, still need to use <=
        let score_to_beat = path.len() - 100;

        let mut hacks_beat = 0;

        for (index, coord) in path.iter().enumerate() {
            // dbg!(index);

            let sprawling = path.iter().skip(index + 100).filter_map(|other| {
                let man_dist = other.0.abs_diff(coord.0) + other.1.abs_diff(coord.1);

                if man_dist <= 20 {
                    Some((man_dist, *other))
                } else {
                    None
                }
            });

            // Remember to skip the 1st which is where we started from
            for (cheating_score, start) in sprawling {
                let end_index = indexes[start.1 * self.width + start.0];

                if end_index != usize::MAX {
                    let score = index + cheating_score + (path.len() - end_index);

                    if score <= score_to_beat {
                        hacks_beat += 1;
                    }
                }
            }
        }

        hacks_beat
    }
}

pub fn part1() {
    let code = Code::from_str(INPUT);

    let path = code.path_lmao();

    dbg!(path.len());

    // score is len - 1; to beat is at least 100 less than that
    // even accounting for this +1 from including start pos, still need to use <=
    let score_to_beat = path.len() - 100;

    let hack_count = code.hacks_for(&path).filter(|score| *score <= score_to_beat).count();

    dbg!(hack_count);
}

pub fn part2() {
    let code = Code::from_str(INPUT);

    dbg!(code.op_hacks());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let code = Code::from_str(
            "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
",
        );

        let path = code.path_lmao();

        assert_eq!(path.len(), 85);

        let hack_count = code.hacks_for(&path).filter(|score| *score < 84).count();

        dbg!(hack_count);

        let score_to_beat = path.len() - 64;

        let hack_count = code.hacks_for(&path).filter(|score| *score <= score_to_beat).count();

        dbg!(hack_count);
    }
}
