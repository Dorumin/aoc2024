use std::{
    collections::HashMap,
    fmt::{Display, Write},
};

use itertools::Itertools;

const INPUT: &str = include_str!("../../../inputs/day21.txt");

type Coord = (usize, usize);

struct Keypad {
    ty: KeypadType,
    map: HashMap<Key, Coord>,
}

enum KeypadType {
    Numeric,
    Directional,
}

struct Keychain {
    keypads: Vec<Keypad>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Key {
    Panic,
    Commit,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_key(self) -> Key {
        match self {
            Direction::Up => Key::Up,
            Direction::Down => Key::Down,
            Direction::Left => Key::Left,
            Direction::Right => Key::Right,
        }
    }
}

impl Key {
    fn from_char(c: char) -> Self {
        match c {
            ' ' => Self::Panic,
            'A' => Self::Commit,
            '0' => Self::Zero,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!(),
        }
    }

    fn to_char(self) -> char {
        match self {
            Self::Panic => ' ',
            Self::Commit => 'A',
            Self::Zero => '0',
            Self::One => '1',
            Self::Two => '2',
            Self::Three => '3',
            Self::Four => '4',
            Self::Five => '5',
            Self::Six => '6',
            Self::Seven => '7',
            Self::Eight => '8',
            Self::Nine => '9',
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }
}

impl Keypad {
    fn new(ty: KeypadType) -> Self {
        let mut map = HashMap::new();

        match ty {
            KeypadType::Numeric => {
                map.insert(Key::Seven, (0, 0));
                map.insert(Key::Eight, (1, 0));
                map.insert(Key::Nine, (2, 0));
                map.insert(Key::Four, (0, 1));
                map.insert(Key::Five, (1, 1));
                map.insert(Key::Six, (2, 1));
                map.insert(Key::One, (0, 2));
                map.insert(Key::Two, (1, 2));
                map.insert(Key::Three, (2, 2));
                map.insert(Key::Panic, (0, 3));
                map.insert(Key::Zero, (1, 3));
                map.insert(Key::Commit, (2, 3));
            }
            KeypadType::Directional => {
                map.insert(Key::Panic, (0, 0));
                map.insert(Key::Up, (1, 0));
                map.insert(Key::Commit, (2, 0));
                map.insert(Key::Left, (0, 1));
                map.insert(Key::Down, (1, 1));
                map.insert(Key::Right, (2, 1));
            }
        }

        Self { ty, map }
    }

    fn numeric() -> Self {
        Self::new(KeypadType::Numeric)
    }

    fn directional() -> Self {
        Self::new(KeypadType::Directional)
    }

    fn find(&self, key: Key) -> Coord {
        *self.map.get(&key).unwrap()
    }

    fn paths_to(
        &self,
        (from_x, from_y): Coord,
        (to_x, to_y): Coord,
    ) -> impl Iterator<Item = Vec<Direction>> + '_ {
        let x_diff = from_x.abs_diff(to_x);
        let y_diff = from_y.abs_diff(to_y);
        let hmove = if from_x > to_x {
            Direction::Left
        } else {
            Direction::Right
        };
        let vmove = if from_y > to_y {
            Direction::Up
        } else {
            Direction::Down
        };

        let mut moves = vec![hmove; x_diff + y_diff];
        moves[x_diff..].fill(vmove);
        // moves.extend(std::iter::repeat(vmove).take(y_diff));

        // permutations, flat map, range? whatever, itertools
        moves
            .into_iter()
            .permutations(x_diff + y_diff)
            .unique()
            .filter(move |p| !self.path_panics((from_x, from_y), p))
    }

    fn path_panics(&self, from: Coord, path: &[Direction]) -> bool {
        let mut pos = from;

        path.iter().any(|dir| {
            pos = match dir {
                Direction::Up => (pos.0, pos.1 - 1),
                Direction::Down => (pos.0, pos.1 + 1),
                Direction::Left => (pos.0 - 1, pos.1),
                Direction::Right => (pos.0 + 1, pos.1),
            };

            self.find(Key::Panic) == pos
        })
    }
}

impl Keychain {
    fn new(keypads: Vec<Keypad>) -> Self {
        Self { keypads }
    }

    fn shortest_translation(
        &self,
        index: usize,
        keypad_arm: Key,
        to_key: Key,
        cache: &mut HashMap<(usize, Key, Key), Inputs>,
    ) -> Inputs {
        if let Some(best) = cache.get(&(index, keypad_arm, to_key)) {
            return best.clone();
        }

        let keypad = &self.keypads[index];

        let from = keypad.find(keypad_arm);
        let to = keypad.find(to_key);

        if from == to {
            return Inputs::from_str("A");
        }

        let mut best_path: Option<Inputs> = None;

        for path in keypad.paths_to(from, to) {
            // todo: opt alloc
            let mut path: Vec<_> = path.into_iter().map(|dir| dir.to_key()).collect();
            path.push(Key::Commit);

            let path = if index == self.keypads.len() - 1 {
                Inputs { keys: path }
            } else {
                self.shortest_keypass(Inputs { keys: path }, index + 1, cache)
            };

            if let Some(ref best) = best_path {
                if best.keys.len() > path.keys.len() {
                    best_path.replace(path);
                }
            } else {
                best_path.replace(path);
            }
        }

        cache.insert(
            (index, keypad_arm, to_key),
            best_path.as_ref().unwrap().clone(),
        );

        best_path.unwrap()
    }

    fn shortest_keypass(
        &self,
        commands: Inputs,
        start: usize,
        cache: &mut HashMap<(usize, Key, Key), Inputs>,
    ) -> Inputs {
        let mut keypad_arm = Key::Commit;
        let mut final_keypresses = Inputs::from_str("");

        for key in commands.keys {
            let sequencing = self.shortest_translation(start, keypad_arm, key, cache);

            final_keypresses.keys.extend(sequencing.keys);
            keypad_arm = key;
        }

        final_keypresses
    }
}

#[derive(Clone)]
struct Inputs {
    keys: Vec<Key>,
}

impl Inputs {
    fn from_str(code: &str) -> Self {
        Self {
            keys: code.chars().map(Key::from_char).collect(),
        }
    }
}

impl Display for Inputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for key in self.keys.iter() {
            f.write_char(key.to_char()).unwrap();
        }

        Ok(())
    }
}

pub fn part1() {
    let depressurized_numpad = Keypad::numeric();
    let irradiated_keypad = Keypad::directional();
    let freezing_keypad = Keypad::directional();
    // this is where we type from!
    // let mut chronicled_keypad = Keypad::directional();

    let chain = Keychain::new(vec![
        depressurized_numpad,
        irradiated_keypad,
        freezing_keypad,
    ]);

    let codes = INPUT.lines();

    let mut total = 0;
    let mut cache = HashMap::new();

    for code in codes {
        let inputs = Inputs::from_str(code);

        let result = chain.shortest_keypass(inputs, 0, &mut cache);

        dbg!(result.keys.len());

        let nummy: usize = code.trim_end_matches(|c: char| c.is_alphabetic()).parse().unwrap();
        let score = result.keys.len() * nummy;

        total += score;
    }

    dbg!(total);
}

pub fn part2() {
    let mut keypads = vec![Keypad::numeric()];

    keypads.extend(std::iter::repeat_with(Keypad::directional).take(21));

    let chain = Keychain::new(keypads);

    let codes = INPUT.lines();

    let mut total = 0;
    let mut cache = HashMap::new();

    for code in codes {
        let inputs = Inputs::from_str(code);

        let result = chain.shortest_keypass(inputs, 0, &mut cache);

        dbg!(result.keys.len());

        let nummy: usize = code.trim_end_matches(|c: char| c.is_alphabetic()).parse().unwrap();
        let score = result.keys.len() * nummy;

        total += score;
    }

    dbg!(total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut depressurized_numpad = Keypad::numeric();
        let mut irradiated_keypad = Keypad::directional();
        let mut freezing_keypad = Keypad::directional();
        let mut chronicled_keypad = Keypad::directional();

        let mut commands: Vec<_> = "029A".chars().map(Key::from_char).collect();

        dbg!(depressurized_numpad
            .paths_to(
                depressurized_numpad.find(Key::Commit),
                depressurized_numpad.find(Key::Four)
            )
            .collect::<Vec<_>>());
    }
}
