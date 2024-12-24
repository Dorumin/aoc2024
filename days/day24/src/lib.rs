use std::collections::HashMap;

use regex::Regex;

const INPUT: &str = include_str!("../../../inputs/day24.txt");

#[derive(Debug)]
struct Wirings<'a> {
    wires: HashMap<&'a str, bool>,
    terms: Vec<Term<'a>>,
}

#[derive(Debug)]
struct Term<'a> {
    a: &'a str,
    b: &'a str,
    out: &'a str,
    op: Op,
}

#[derive(Debug)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn from_str(op: &str) -> Op {
        match op {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => unreachable!(),
        }
    }

    fn solve(&self, operand_a: bool, operand_b: bool) -> bool {
        match self {
            Op::And => operand_a && operand_b,
            Op::Or => operand_a || operand_b,
            Op::Xor => operand_a ^ operand_b,
        }
    }
}

impl<'a> Wirings<'a> {
    fn from_str(input: &'a str) -> Self {
        let seed_regex = Regex::new(r"^(\w+): ([01])$").unwrap();
        let term_regex = Regex::new(r"^(\w+) (AND|OR|XOR) (\w+) -> (\w+)").unwrap();
        let mut lines = input.lines();
        let mut wires = HashMap::new();
        let mut terms = Vec::new();

        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            let seed_caps = seed_regex.captures(line).unwrap();
            let seed_wire = seed_caps.get(1).unwrap().as_str();
            let seed_value = seed_caps.get(2).unwrap().as_str() == "1";

            wires.insert(seed_wire, seed_value);
        }

        for line in &mut lines {
            let term_caps = term_regex.captures(line).unwrap();
            let term_dep_a = term_caps.get(1).unwrap().as_str();
            let term_op = term_caps.get(2).unwrap().as_str();
            let term_dep_b = term_caps.get(3).unwrap().as_str();
            let term_wire = term_caps.get(4).unwrap().as_str();

            terms.push(Term {
                a: term_dep_a,
                b: term_dep_b,
                out: term_wire,
                op: Op::from_str(term_op),
            });
        }

        Self { wires, terms }
    }

    fn solve(&mut self) {
        loop {
            let initial_solved = self.wires.len();

            self.terms.retain(|term| {
                let Some(a) = self.wires.get(term.a) else {
                    return true;
                };
                let Some(b) = self.wires.get(term.b) else {
                    return true;
                };

                self.wires.insert(term.out, term.op.solve(*a, *b));

                false
            });

            let final_solved = self.wires.len();
            let delta_solved = final_solved - initial_solved;

            eprintln!("solved {delta_solved} terms");

            if delta_solved == 0 {
                break;
            }
        }
    }

    fn z_sum(&self) -> u64 {
        let mut key_value: Vec<_> = self.wires.iter().filter(|(k, _)| k.starts_with("z")).collect();

        key_value.sort_by_key(|(k, _)| *k);
        key_value.reverse(); // It's the other way around????? For some reason?

        // dbg!(&key_value);

        key_value.iter().fold(0, |bits, (_, v)| (bits << 1) + **v as u64)
    }
}

pub fn part1() {
    let mut wirings = Wirings::from_str(INPUT);

    wirings.solve();

    dbg!(wirings.z_sum());
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let mut wirings = Wirings::from_str(
            "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02",
        );

        wirings.solve();
        dbg!(&wirings.terms);
        dbg!(wirings.z_sum());
    }

    #[test]
    fn example_one_tai_lunger() {
        let mut wirings = Wirings::from_str(
            "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj",
        );

        wirings.solve();
        dbg!(&wirings.terms);
        dbg!(wirings.z_sum());
    }
}
