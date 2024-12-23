use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../../inputs/day23.txt");

#[derive(Debug)]
struct LanParty<'a> {
    map: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> LanParty<'a> {
    fn from_str(input: &'a str) -> Self {
        let mut map: HashMap<_, Vec<_>> = HashMap::new();

        for line in input.lines() {
            let (left, right) = line.split_once("-").unwrap();
            assert_ne!(left, right);

            map.entry(left).or_default().push(right);
            map.entry(right).or_default().push(left);
        }

        Self { map }
    }

    fn sets(&self, lenis: usize) -> HashSet<Vec<&'a str>> {
        let mut sets = HashSet::new();

        fn explore_seq<'a>(
            seq: Vec<&'a str>,
            rest: &[&'a str],
            map: &HashMap<&'a str, Vec<&'a str>>,
            set: &mut HashSet<Vec<&'a str>>,
            remaining: usize,
        ) {
            for next in rest {
                if seq.contains(next) {
                    continue;
                }

                let mut nseq = seq.clone();
                nseq.push(next);

                if remaining == 0 {
                    nseq.sort();
                    set.insert(nseq);
                } else {
                    let nrest = map.get(next).unwrap();

                    explore_seq(nseq, nrest, map, set, remaining - 1);
                }
            }
        }

        for (&start, rest) in self.map.iter() {
            let seq = vec![start];

            explore_seq(seq, rest, &self.map, &mut sets, lenis - 2);
        }

        sets
    }

    fn group_sets(&'a self, lenis: usize) -> HashSet<Vec<&'a str>> {
        let mut sets = HashSet::new();

        fn explore_seq<'a>(
            seq: Vec<&'a str>,
            rest: &[&'a str],
            map: &HashMap<&'a str, Vec<&'a str>>,
            set: &mut HashSet<Vec<&'a str>>,
            remaining: usize,
        ) {
            for next in rest {
                if seq.contains(next) {
                    continue;
                }

                let mut nseq = seq.clone();
                nseq.push(next);

                if remaining == 0 {
                    if nseq.iter().all(|conn| {
                        nseq.iter()
                            .all(|other| other == conn || map.get(other).unwrap().contains(conn))
                    }) {
                        nseq.sort();
                        set.insert(nseq);
                    }
                } else {
                    let nrest = map.get(next).unwrap();

                    explore_seq(nseq, nrest, map, set, remaining - 1);
                }
            }
        }

        for (&start, rest) in self.map.iter() {
            let seq = vec![start];

            explore_seq(seq, rest, &self.map, &mut sets, lenis - 2);
        }

        sets
    }
}

pub fn part1() {
    let party = LanParty::from_str(INPUT);
    let mut sets_of_three = party.group_sets(3);

    // filter the trios to a t
    sets_of_three
        .retain(|menage_a_trois| menage_a_trois.iter().any(|computer| computer.starts_with("t")));

    dbg!(sets_of_three.len());
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let party = LanParty::from_str(
            "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn",
        );

        // dbg!(party);
        dbg!(party.group_sets(3));
        dbg!(party.group_sets(3).len());
    }
}