const INPUT: &str = include_str!("../../../inputs/day7.txt");

struct Bridge {
    equations: Vec<Equation>,
}

struct Equation {
    ideal: i64,
    atoms: Vec<i64>,
}

#[derive(Clone)]
enum Operators {
    Add,
    Mul,
}

impl Operators {
    fn next(&self) -> Option<Operators> {
        match self {
            Operators::Add => Some(Operators::Mul),
            Operators::Mul => None,
        }
    }
}

impl Bridge {
    fn from_str(input: &str) -> Self {
        Self {
            equations: input.lines().map(Equation::from_line).collect(),
        }
    }

    fn solvable_sum(&self) -> i64 {
        self.equations
            .iter()
            .filter(|e| e.is_solvable())
            .fold(0, |sum, eq| sum + eq.ideal)
    }
}

impl Equation {
    fn from_line(line: &str) -> Self {
        let (ideal, atoms) = line.split_once(": ").unwrap();
        let ideal = ideal.parse().unwrap();
        let atoms = atoms.split_whitespace().map(|n| n.parse().unwrap()).collect();

        Self { ideal, atoms }
    }

    fn try_reduce_ops(ops: &mut [Operators]) -> bool {
        let mut reduced = None;

        for i in (0..ops.len()).rev() {
            if let Some(new) = ops[i].next() {
                reduced = Some(i);
                ops[i] = new;
                break;
            }
        }

        if let Some(start) = reduced {
            ops.iter_mut().skip(start + 1).for_each(|op| *op = Operators::Add);
            true
        } else {
            false
        }
    }

    fn is_solvable(&self) -> bool {
        assert!(self.atoms.len() > 1);

        let mut ops = vec![Operators::Add; self.atoms.len() - 1];

        loop {
            let mut start = self.atoms[0];
            self.atoms
                .iter()
                .skip(1)
                .enumerate()
                .for_each(|(index, atom)| match ops[index] {
                    Operators::Add => start += atom,
                    Operators::Mul => start *= atom,
                });

            if start == self.ideal {
                return true;
            }

            if !Equation::try_reduce_ops(&mut ops) {
                break;
            }
        }

        false
    }
}

pub fn part1() {
    let bridge = Bridge::from_str(INPUT);

    dbg!(bridge.solvable_sum());
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let bridge = Bridge::from_str(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );

        dbg!(bridge.solvable_sum());
    }
}
