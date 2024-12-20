use std::{cmp::Reverse, collections::BinaryHeap};

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

    fn index_mut(&mut self, (x, y): Coord) -> &mut Tile {
        &mut self.tiles[y * self.width + x]
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

    // Pathfinding 2.0
    fn dijkstra(&self, start: Coord) -> Vec<Coord> {
        self.dijkstra_dirty(start, &mut Vec::new(), &mut Vec::new())
    }

    fn dijkstra_dirty(
        &self,
        start: Coord,
        costs: &mut Vec<Option<i32>>,
        parents: &mut Vec<Option<Coord>>,
    ) -> Vec<Coord> {
        costs.truncate(0);
        parents.truncate(0);
        costs.resize(self.width * self.height, None);
        parents.resize(self.width * self.height, None);

        let end = self.end();

        let mut pqueue = BinaryHeap::new();

        pqueue.push((Reverse(0), start));
        costs[start.1 * self.width + start.0] = Some(0);

        while let Some((Reverse(cost), coord)) = pqueue.pop() {
            if coord == end {
                break;
            }

            for next in self.adj(coord) {
                if matches!(self.index(next), Tile::Wall) {
                    continue;
                }

                let new_cost = cost + 1;
                let existing_cost = costs[next.1 * self.width + next.0];
                let is_cheaper = existing_cost.is_none() || existing_cost.unwrap() > new_cost;

                if is_cheaper {
                    costs[next.1 * self.width + next.0] = Some(new_cost);
                    parents[next.1 * self.width + next.0] = Some(coord);
                    pqueue.push((Reverse(new_cost), next));
                }
            }
        }

        let mut pathximus = vec![];
        let mut cur = end;

        while let Some(p) = parents[cur.1 * self.width + cur.0] {
            pathximus.push(cur);
            cur = p;
        }

        // Insert start at the end (to start) so we account it when hacking
        pathximus.push(start);

        pathximus.reverse();

        pathximus
    }

    fn hacks_for<'a>(&'a self, path: &'a [Coord]) -> impl Iterator<Item = usize> + 'a {
        path.iter().enumerate().flat_map(move |(index, coord)| {
            let adjacents = self.adj(*coord).flat_map(|adj| self.adj(adj)).filter(move |skipped| {
                !path[0..index].contains(skipped) && !matches!(self.index(*skipped), Tile::Wall)
            });

            adjacents.map(move |adj| self.dijkstra(adj).len() + index + 2)
        })
    }
}

pub fn part1() {
    let code = Code::from_str(INPUT);

    let path = code.dijkstra(code.start());

    dbg!(path.len());

    // score is len - 1; to beat is at least 100 less than that
    // even accounting for this +1 from including start pos, still need to use <=
    let score_to_beat = path.len() - 100;

    let hack_count = code.hacks_for(&path).filter(|score| *score <= score_to_beat).count();

    dbg!(hack_count);
}

pub fn part2() {}

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

        let path = code.dijkstra(code.start());

        assert_eq!(path.len(), 85);

        let hack_count = code.hacks_for(&path).filter(|score| *score < 84).count();

        dbg!(hack_count);

        let score_to_beat = path.len() - 64;

        let hack_count = code.hacks_for(&path).filter(|score| *score <= score_to_beat).count();

        dbg!(hack_count);
    }
}
