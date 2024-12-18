use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

const INPUT: &str = include_str!("../../../inputs/day18.txt");

type Coord = (usize, usize);

#[derive(Debug)]
struct Ram {
    width: usize,
    height: usize,
    fallen: usize,
    bytes: Vec<Coord>,
    tiles: Vec<Tile>,
}

#[derive(Clone, Copy, Debug)]
enum Tile {
    Free,
    Byte,
}

impl Ram {
    fn from_str(input: &str, width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            fallen: 0,
            bytes: input
                .lines()
                .map(|line| {
                    let (x, y) = line.split_once(",").unwrap();
                    let (x, y) = (x.parse().unwrap(), y.parse().unwrap());

                    (x, y)
                })
                .collect(),
            tiles: vec![Tile::Free; width * height],
        }
    }

    fn index(&self, (x, y): Coord) -> Tile {
        self.tiles[y * self.width + x]
    }

    fn index_mut(&mut self, (x, y): Coord) -> &mut Tile {
        &mut self.tiles[y * self.width + x]
    }

    fn index_to_coord(&self, index: usize) -> Coord {
        (index % self.width, index / self.width)
    }

    fn fall(&mut self, count: usize) {
        // Sneaky borrow
        let mut bytes = std::mem::take(&mut self.bytes);
        for byte in bytes.iter().cloned().skip(self.fallen).take(count) {
            *self.index_mut(byte) = Tile::Byte;
        }

        std::mem::swap(&mut bytes, &mut self.bytes);

        self.fallen += count;
    }

    fn adj(&self, (x, y): Coord) -> impl Iterator<Item = (Coord)> {
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

    fn start(&self) -> Coord {
        // habit
        (0, 0)
    }

    fn end(&self) -> Coord {
        // stay high
        (self.width - 1, self.height - 1)
    }

    // Pathfinding 2.0
    fn dijkstra(&self) -> Vec<Coord> {
        let start = self.start();
        let end = self.end();

        let mut pqueue = BinaryHeap::new();
        let mut costos = HashMap::new();
        let mut daddies = HashMap::new();
        pqueue.push((Reverse(0), start));
        costos.insert(start, 0);

        while let Some((Reverse(cost), coord)) = pqueue.pop() {
            if coord == end {
                break;
            }

            for next in self.adj(coord) {
                if matches!(self.index(next), Tile::Byte) {
                    // oof, hit the wall
                    continue;
                }

                let new_cost = cost + 1;
                let existing_cost = costos.get(&next);
                let is_cheaper = existing_cost.is_none() || *existing_cost.unwrap() > new_cost;

                if is_cheaper {
                    costos.insert(next, new_cost);
                    daddies.insert(next, coord);
                    pqueue.push((Reverse(new_cost), next));
                }
            }
        }

        let mut pathximus = vec![];
        let mut cur = end;
        while let Some(p) = daddies.get(&cur) {
            pathximus.push(cur);
            cur = *p;
        }

        pathximus.reverse();

        pathximus
    }
}

pub fn part1() {
    // 71 fucking tiles because it goes from 0..70
    let mut ram = Ram::from_str(INPUT, 71, 71);

    ram.fall(1024);

    dbg!(ram.dijkstra().len());
}

pub fn part2() {
    // 71 fucking tiles because it goes from 0..70
    let mut ram = Ram::from_str(INPUT, 71, 71);

    // good start
    ram.fall(1024);

    let mut ruta = ram.dijkstra();

    for _ in 0.. {
        let fuckbyte = ram.bytes[ram.fallen];
        ram.fall(1);

        if !ruta.contains(&fuckbyte) {
            continue;
        }

        ruta = ram.dijkstra();

        if ruta.is_empty() {
            dbg!(ram.fallen);
            dbg!(fuckbyte);

            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut ram = Ram::from_str(
            "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
",
            7,
            7,
        );

        ram.fall(12);

        dbg!(ram.dijkstra().len());
    }
}
