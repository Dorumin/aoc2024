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

    fn sprawling(&self, (x, y): Coord, max_dist: usize) -> impl Iterator<Item = (usize, Coord)> {
        let d = max_dist as isize;
        let w = self.width as isize;
        let h = self.height as isize;

        // We map our diamond indexes horizontally from negative to positive inc. range...
        (-d..=d).flat_map(move |dx| {
            // ... and we map our vertical diamond indexes, subtracting the absolute value
            // of our horizontal index (so we get a by-design manhattan limit)
            ((-d + dx.abs())..(d - dx.abs() + 1)).filter_map(move |dy| {
                let next = ((x as isize) + dx, (y as isize) + dy);

                if next.0 < 0 || next.0 >= w || next.1 < 0 || next.1 >= h {
                    None
                } else {
                    let coord = (next.0 as usize, next.1 as usize);
                    let distance = dx.abs() + dy.abs();

                    Some((distance as usize, coord))
                }
            })
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
                .sprawling(current, 1)
                .find(|(_, next)| {
                    !matches!(self.index(*next), Tile::Wall)
                        && *next != previous
                        && *next != current
                })
                .unwrap();

            coords.push(current);
            previous = current;
            current = next.1;
        }

        coords.push(current);

        // only verify all free tiles are in path in debug builds
        #[cfg(debug_assertions)]
        for (index, tile) in self.tiles.iter().enumerate() {
            if !matches!(tile, Tile::Wall) {
                let coord = self.index_to_coord(index);

                assert!(coords.contains(&coord));
            }
        }

        coords
    }

    fn count_worthwhile_cheats(&self, min_savings: usize, max_distance: usize) -> i32 {
        let path = self.path_lmao();

        // Mapping beats a 9k cell scan
        let mut indexes = vec![usize::MAX; self.width * self.height];
        for (index, coord) in path.iter().enumerate() {
            indexes[coord.1 * self.width + coord.0] = index;
        }

        // score is len - 1; to beat is at least [min_savings] less than that
        // even accounting for this +1 from including start pos, still need to use <=
        let score_to_beat = path.len() - min_savings;

        let mut hacks_beat = 0;

        for (index, coord) in path.iter().enumerate() {
            for (cheating_score, start) in self.sprawling(*coord, max_distance) {
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

    dbg!(code.count_worthwhile_cheats(100, 2));
}

pub fn part2() {
    let code = Code::from_str(INPUT);

    dbg!(code.count_worthwhile_cheats(100, 20));
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

        let cheat_count = code.count_worthwhile_cheats(64, 2);

        assert_eq!(cheat_count, 1);
    }
}
