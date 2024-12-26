use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

use regex::Regex;

const INPUT: &str = include_str!("../../../inputs/day24.txt");

#[derive(Debug, Clone)]
struct Wirings<'a> {
    wires: HashMap<&'a str, bool>,
    terms: Vec<Term<'a>>,
    remaining_terms: Vec<Term<'a>>,
}

#[derive(Debug, Clone)]
struct Term<'a> {
    a: &'a str,
    b: &'a str,
    out: &'a str,
    op: Op,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn to_str(&self) -> &'static str {
        match self {
            Op::And => "&",
            Op::Or => "|",
            Op::Xor => "^",
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

        Self {
            wires,
            terms: terms.clone(),
            remaining_terms: terms,
        }
    }

    fn solve(&mut self) {
        loop {
            let initial_solved = self.wires.len();

            self.remaining_terms.retain(|term| {
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

            // eprintln!("solved {delta_solved} terms");

            if delta_solved == 0 {
                break;
            }
        }
    }

    fn unsolve(&mut self) {
        // lmao you won't catch me dead with immutability
        self.wires.retain(|k, _| k.starts_with(['x', 'y']));
        self.remaining_terms = self.terms.clone();
    }

    fn sum_bits(&self, prefix: char) -> u64 {
        let mut bits: Vec<_> = self.wires.iter().filter(|kv| kv.0.starts_with(prefix)).collect();

        // Sort bits the other way around - 00 = most significant bit
        bits.sort_by_key(|(k, _)| Reverse(*k));

        bits.iter().fold(0, |bits, (_, v)| (bits << 1) + **v as u64)
    }

    fn add_seeds(&self) -> u64 {
        self.sum_bits('x') + self.sum_bits('y')
    }

    fn term_depth(&self, t: &Term<'a>) -> u32 {
        fn recourse(wirings: &Wirings, term: &Term) -> u32 {
            let a_count = if wirings.wires.contains_key(term.a) {
                1
            } else {
                let term_a = wirings.terms.iter().find(|t| t.out == term.a).unwrap();

                recourse(wirings, term_a)
            };
            let b_count = if wirings.wires.contains_key(term.b) {
                1
            } else {
                let term_b = wirings.terms.iter().find(|t| t.out == term.b).unwrap();

                recourse(wirings, term_b)
            };

            a_count + b_count
        }

        recourse(self, t)
    }

    fn term_depth_re(&self, t: &Term<'a>) -> Option<u32> {
        fn recourse<'a>(
            wirings: &'a Wirings,
            term: &'a Term,
            seen: &mut HashSet<&'a str>,
        ) -> Option<u32> {
            if !seen.insert(term.out) {
                return None;
            }

            let a_count = if wirings.wires.contains_key(term.a) {
                Some(1)
            } else {
                wirings
                    .terms
                    .iter()
                    .find(|t| t.out == term.a)
                    .and_then(|term_a| recourse(wirings, term_a, seen))
            };

            let b_count = if wirings.wires.contains_key(term.b) {
                Some(1)
            } else {
                wirings
                    .terms
                    .iter()
                    .find(|t| t.out == term.b)
                    .and_then(|term_b| recourse(wirings, term_b, seen))
            };

            // Remove the current term from the seen set after recursion
            seen.remove(term.out);

            match (a_count, b_count) {
                (Some(a), Some(b)) => Some(a + b),
                _ => None, // Propagate failure if any subterm fails
            }
        }

        recourse(self, t, &mut HashSet::new())
    }

    fn get_z_swaps(&mut self) -> Vec<(usize, usize)> {
        fn desired_length(z_term_index: usize) -> u32 {
            match z_term_index {
                0 => 2,
                1 => 4,
                n => n as u32 * 4,
            }
        }

        let mut swaps = vec![];

        loop {
            let mut z_terms: Vec<_> = self
                .terms
                .iter()
                .enumerate()
                .filter(|(_, term)| term.out.starts_with("z"))
                .collect();

            z_terms.sort_by_key(|(_, term)| term.out);

            let mut swap: Option<(usize, usize)> = None;

            for index in 0..z_terms.len() {
                let (z_term_index, term) = z_terms[index];
                // eprintln!("{} {:?} {:?}", term.out, term.op, self.term_depth_re(term));

                // let z_depth: u64 = term.out[1..].parse().unwrap(); equal to index
                let mut tags = Vec::new();
                let proper_length = desired_length(index);

                // hardcoding our least significant carry bit which is not computed with xor
                if term.op != Op::Xor && term.out != "z45" {
                    tags.push("nxor");

                    // eprintln!("nxor {z_term_index} z{index}");
                    let replacements: Vec<_> = self
                        .terms
                        .iter()
                        .enumerate()
                        .filter(|(_, term)| {
                            // eprintln!("{:?} {:?}", self.term_depth(term), self.term_depth_re(term));

                            term.op == Op::Xor
                                && !term.out.starts_with('z')
                                && self.term_depth_re(term) == Some(proper_length)
                        })
                        .collect();

                    // dbg!(replacements.len());

                    if replacements.len() > 1 {
                        eprintln!("{} for {} {}", replacements.len(), proper_length, term.out);
                        // self.debug_dependencies(term.out);
                        // panic!();
                    }

                    if replacements.is_empty() {
                        continue;
                    }

                    let (replacement_index, _) = replacements[0];

                    swap = Some((z_term_index, replacement_index));

                    break;
                }

                // if self.term_depth(term) != proper_length {
                //     tags.push("wrong_length");
                // }

                // eprintln!(
                //     "{index} for {} {{{}}}",
                //     self.term_depth(term),
                //     tags.join(",")
                // );

                // assert_eq!(term.op, Op::Xor, "z-terms are always xors");
            }

            if let Some((i, j)) = swap {
                swaps.push((i, j));

                // eprintln!(
                //     "swapping {i} and {j}; {} and {}",
                //     self.terms[i].out, self.terms[j].out
                // );

                let stomp = self.terms[i].out;

                self.terms[i].out = self.terms[j].out;
                self.terms[j].out = stomp;

                continue;
            } else {
                let fucked_terms = z_terms
                    .iter()
                    .enumerate()
                    .map(|(index, (_z_index, term))| {
                        (index, term, self.term_depth_re(term), desired_length(index))
                    })
                    .filter(|(index, term, depth, desired)| {
                        term.out != "z45" && *depth != Some(*desired)
                    })
                    .collect::<Vec<_>>();

                if !fucked_terms.is_empty() {
                    // eprintln!("fucked");
                    // eprintln!("{:?}", fucked_terms);
                }
                break;
            }
        }

        swaps
    }

    fn find_first_fucked_bit(&mut self) -> u64 {
        let original_terms = self.terms.clone();

        let swaps = self.get_z_swaps();
        let zs_to_swap: Vec<_> = swaps.iter().map(|pair| pair.0).collect();
        let real_sum = self.add_seeds();

        self.solve();
        eprintln!("{swaps:?}");
        eprintln!("{zs_to_swap:?}");
        dbg!(self.sum_bits('z'));

        self.unsolve();

        // return 0;

        for i in 0..self.terms.len() {
            // if zs_to_swap.contains(&i) {
            //     continue;
            // }

            for j in (i + 1)..self.terms.len() {
                // if zs_to_swap.contains(&i) {
                //     continue;
                // }

                // eprintln!("{i} {j}");
                self.terms = original_terms.clone();
                self.remaining_terms = original_terms.clone();
                self.wires.retain(|k, _| k.starts_with(['x', 'y']));
                let stomp = self.terms[i].out;
                self.terms[i].out = self.terms[j].out;
                self.terms[j].out = stomp;
                self.remaining_terms[i].out = self.remaining_terms[j].out;
                self.remaining_terms[j].out = stomp;
                // self.terms.swap(i, j);
                // self.remaining_terms.swap(i, j);

                let _new_swaps = self.get_z_swaps();

                // assert_eq!(new_swaps.len(), swaps.len());

                self.solve();

                let at_least_there_was_an_attempt_sum = self.sum_bits('z');

                if real_sum == at_least_there_was_an_attempt_sum {
                    eprintln!("real or fake? {i} {j}");
                    panic!();
                }

                // self.unsolve();

                // unswap
                // self.terms.swap(i, j);
            }
        }

        // self.terms = original_terms.clone();

        // self.remaining_terms = self.terms.clone();

        // self.solve();
        // let real = self.add_seeds();
        // let fake = self.sum_bits('z');

        // self.unsolve();
        // eprintln!("{real:0b}");
        // eprintln!("{fake:0b}");

        0
    }

    fn garganta_profunda(&'a self, term: &'a Term<'a>) -> Vec<Vec<&'a Term<'a>>> {
        let mut seen = HashSet::new();
        let mut working_set = vec![term];
        let mut levels = vec![];

        while !working_set.is_empty() {
            let mut next_working_set = vec![];
            let mut level = vec![];

            for term in working_set {
                if !seen.insert(term.out) {
                    return vec![];
                }

                if !self.wires.contains_key(term.a) {
                    next_working_set.push(self.terms.iter().find(|t| t.out == term.a).unwrap());
                };
                if !self.wires.contains_key(term.b) {
                    next_working_set.push(self.terms.iter().find(|t| t.out == term.b).unwrap());
                };

                level.push(term);
            }

            levels.push(level);
            working_set = next_working_set;
        }

        levels
    }

    fn z_terms(&self) -> Vec<(usize, &Term)> {
        let mut z_terms: Vec<_> = self
            .terms
            .iter()
            .enumerate()
            .filter(|(_, term)| term.out.starts_with("z"))
            .collect();

        z_terms.sort_by_key(|(_, term)| term.out);

        z_terms
    }

    fn swap_term_outputs(&mut self, i: usize, j: usize) {
        let stomp = self.terms[i].out;
        self.terms[i].out = self.terms[j].out;
        self.terms[j].out = stomp;
        self.remaining_terms[i].out = self.remaining_terms[j].out;
        self.remaining_terms[j].out = stomp;
    }

    fn fix_fuck(
        &mut self,
        swaps: &[(usize, usize)],
        min_bad: usize,
    ) -> Option<Vec<(usize, usize)>> {
        if swaps.len() > 4 {
            return None;
        }

        fn desired_term_count(index: usize) -> usize {
            match index {
                0 => 1,
                1 => 3,
                _ => index * 4 - 1,
            }
        }

        fn validate_term_levels<'a>(index: usize, levels: &[Vec<&'a Term>]) -> Vec<&'a Term<'a>> {
            assert!(
                !levels.is_empty(),
                "Empty levels; do not call this function with infinite cycles"
            );

            let mut bad_terms = vec![];

            if index == 44 {
                return bad_terms;
            }

            // El primer nivel es siempre un solo término, que es solo un xor
            // para determinar el valor calorífico del bit zNN.
            // La única excepción es el bit menos significante.
            if index != 44 && levels[0][0].op != Op::Xor {
                bad_terms.push(levels[0][0]);
                // return false;
            }

            // El último término es and/xor
            if levels.len() > 1 {
                let level = levels.last().unwrap();
                let one_and = (level[0].op == Op::And) ^ (level[1].op == Op::And);
                let one_xor = (level[0].op == Op::Xor) ^ (level[1].op == Op::Xor);
                let one_of_both = one_and && one_xor;

                if !one_of_both {
                    bad_terms.push(level[0]);
                    bad_terms.push(level[1]);
                    // return false;
                }
            }

            // Cada par de términos entre el término inicial y el término finalizabliminalmentáculo
            // va a ser un par de computaciones XOR/OR y AND/AND, en ese órden
            for i in (1..(levels.len() - 1)).step_by(2) {
                let first = &levels[i];
                let second = &levels[i + 1];

                let one_or = (first[0].op == Op::Or) ^ (first[1].op == Op::Or);
                let one_xor = (first[0].op == Op::Xor) ^ (first[1].op == Op::Xor);

                let both_ands = second[0].op == Op::And && second[1].op == Op::And;

                let both_terms_ok = both_ands && one_xor;

                if !both_terms_ok {
                    bad_terms.push(first[0]);
                    bad_terms.push(first[1]);
                    bad_terms.push(second[0]);
                    bad_terms.push(second[1]);
                    // return false;
                }
            }

            // if bad_terms.is_empty() {
            bad_terms
            // } else {
            //     levels.iter().flatten().cloned().collect::<Vec<_>>()
            // }
        }

        let dopple = self.clone();
        let z_terms = dopple.z_terms();
        let mut bad = None;
        let indent = swaps.len() * 2;

        for index in 0..z_terms.len() {
            let (z_term_index, term) = z_terms[index];
            let term_levels = dopple.garganta_profunda(term);

            // Infinite loop
            if term_levels.is_empty() {
                return None;
            }

            let term_count = term_levels.iter().fold(0, |sum, its| sum + its.len());

            let valid_term_counts = term_count == desired_term_count(index);
            if !valid_term_counts {
                return None;
            }

            let bad_terms = validate_term_levels(index, &term_levels);

            if !bad_terms.is_empty() {
                // eprintln!(
                //     "{:indent$}bad {} {} {} {}",
                //     "",
                //     term.out,
                //     bad_terms.len(),
                //     index,
                //     min_bad
                // );

                // eprintln!("{min_bad} {index}");
                if index > min_bad {
                    bad = Some((index, bad_terms));
                    break;
                } else {
                    return None;
                }
            }
        }

        // Tenemos términos malos - tenemos que intentar **un** intercambio
        // que haga pasar los tests (y esperemos que no hayan dos)
        if let Some((min_bad, bad_terms)) = bad {
            if swaps.len() == 4 {
                return None;
            }

            let bad_terms: Vec<_> = bad_terms.iter().map(|term| (*term).clone()).collect();

            // dbg!(&bad_terms);

            // eprintln!(
            //     "{:indent$}must swap from {min_bad} one of: {bad_terms:?}",
            //     ""
            // );

            for i in 0..bad_terms.len() {
                let term = bad_terms[i].clone();

                // eprintln!("{:?}", term);

                let term_index = self
                    .terms
                    .iter()
                    .enumerate()
                    .find(|(index, other)| other.out == term.out)
                    .unwrap()
                    .0;

                let mut new_swaps = swaps.to_vec();

                for i in 0..self.terms.len() {
                    if i == term_index
                        || swaps.iter().any(|(x, y)| {
                            *x == i || *x == term_index || *y == i || *y == term_index
                        })
                    {
                        continue;
                    }

                    new_swaps.push((term_index, i));

                    // small important note:
                    // while swapped, `i` refers to the actual term index...
                    self.swap_term_outputs(term_index, i);

                    let term_levels = dopple.garganta_profunda(&self.terms[i]);

                    let term_count = term_levels.iter().fold(0, |sum, its| sum + its.len());

                    let valid_term_counts = term_count == desired_term_count(min_bad);

                    eprintln!("{} {} {}", min_bad, term_count, desired_term_count(min_bad));

                    // eprintln!("{}", term_levels.len());

                    if !term_levels.is_empty() && valid_term_counts {
                        let valid = validate_term_levels(i, &term_levels);

                        if valid.is_empty() {
                            eprintln!(
                                "{:indent$}swap {term_index} with {i}; {} swaps; next scan {}",
                                "",
                                swaps.len(),
                                min_bad
                            );

                            // self.debug_dependencies(term.out);

                            // if let Some(final_swaps) = self.fix_fuck(&new_swaps, min_bad) {
                            //     return Some(final_swaps);
                            // }
                        }
                    }

                    new_swaps.pop();
                    self.swap_term_outputs(term_index, i);
                }
            }

            eprintln!("exited from {} bad terms?", bad_terms.len());
        } else {
            return Some(swaps.to_vec());
        }

        None
    }

    fn one_z_swap(&mut self) -> Option<(usize, usize)> {
        fn desired_length(z_term_index: usize) -> u32 {
            match z_term_index {
                0 => 2,
                1 => 4,
                n => n as u32 * 4,
            }
        }

        let mut z_terms: Vec<_> = self
            .terms
            .iter()
            .enumerate()
            .filter(|(_, term)| term.out.starts_with("z"))
            .collect();

        z_terms.sort_by_key(|(_, term)| term.out);

        let mut swap: Option<(usize, usize)> = None;

        for index in 0..z_terms.len() {
            let (z_term_index, term) = z_terms[index];
            // eprintln!("{} {:?} {:?}", term.out, term.op, self.term_depth_re(term));

            // let z_depth: u64 = term.out[1..].parse().unwrap(); equal to index
            let mut tags = Vec::new();
            let proper_length = desired_length(index);

            // hardcoding our least significant carry bit which is not computed with xor
            if term.op != Op::Xor && term.out != "z45" {
                tags.push("nxor");

                // eprintln!("nxor {z_term_index} z{index}");
                let replacements: Vec<_> = self
                    .terms
                    .iter()
                    .enumerate()
                    .filter(|(_, term)| {
                        // eprintln!("{:?} {:?}", self.term_depth(term), self.term_depth_re(term));

                        term.op == Op::Xor
                            && !term.out.starts_with('z')
                            && self.term_depth_re(term) == Some(proper_length)
                    })
                    .collect();

                // dbg!(replacements.len());

                if replacements.len() > 1 {
                    eprintln!("{} for {} {}", replacements.len(), proper_length, term.out);
                    // self.debug_dependencies(term.out);
                    // panic!();
                }

                if replacements.is_empty() {
                    continue;
                }

                let (replacement_index, _) = replacements[0];

                swap = Some((z_term_index, replacement_index));

                break;
            }

            // if self.term_depth(term) != proper_length {
            //     tags.push("wrong_length");
            // }

            // eprintln!(
            //     "{index} for {} {{{}}}",
            //     self.term_depth(term),
            //     tags.join(",")
            // );

            // assert_eq!(term.op, Op::Xor, "z-terms are always xors");
        }

        if let Some((i, j)) = swap {
            // eprintln!(
            //     "swapping {i} and {j}; {} and {}",
            //     self.terms[i].out, self.terms[j].out
            // );

            self.swap_term_outputs(i, j);

            Some((i, j))
        } else {
            // let fucked_terms = z_terms
            //     .iter()
            //     .enumerate()
            //     .map(|(index, (_z_index, term))| {
            //         (index, term, self.term_depth_re(term), desired_length(index))
            //     })
            //     .filter(|(index, term, depth, desired)| {
            //         term.out != "z45" && *depth != Some(*desired)
            //     })
            //     .collect::<Vec<_>>();

            // if !fucked_terms.is_empty() {
            //     // eprintln!("fucked");
            //     // eprintln!("{:?}", fucked_terms);
            // }

            None
        }
    }

    fn z_indices(&self) -> Vec<usize> {
        let mut z_terms: Vec<_> = self
            .terms
            .iter()
            .enumerate()
            .filter(|(_, term)| term.out.starts_with("z"))
            .collect();

        z_terms.sort_by_key(|(_, term)| term.out);

        let z_term_indices: Vec<_> = z_terms.into_iter().map(|(i, _)| i).collect();

        z_term_indices
    }

    fn shortcut_v2(&mut self) {
        let original_terms = self.terms.clone();

        let swaps = self.get_z_swaps();
        let zs_to_swap: Vec<_> = swaps.iter().map(|pair| pair.0).collect();
        let real_sum = self.add_seeds();

        self.solve();
        let tried_sum = self.sum_bits('z');
        eprintln!("{swaps:?}");
        eprintln!("{zs_to_swap:?}");
        dbg!(real_sum);
        dbg!(tried_sum);

        self.unsolve();

        self.terms = original_terms.clone();
        self.remaining_terms = original_terms.clone();

        let z_term_indices = self.z_indices();

        let mut swaps = 0;

        // Triple any loop is untenable computationally

        // for i in 0..self.terms.len() {
        //     if z_term_indices.contains(&i) {
        //         continue;
        //     }

        //     for j in (i + 1)..self.terms.len() {
        //         if z_term_indices.contains(&j) {
        //             continue;
        //         }

        //         self.terms = original_terms.clone();
        //         self.remaining_terms = original_terms.clone();

        //         self.wires.retain(|k, _| k.starts_with(['x', 'y']));

        //         self.swap_term_outputs(i, j);

        //         let _new_swaps = self.get_z_swaps();

        //         // assert_eq!(new_swaps.len(), swaps.len());

        //         self.solve();

        //         let at_least_there_was_an_attempt_sum = self.sum_bits('z');

        //         if real_sum == at_least_there_was_an_attempt_sum {
        //             eprintln!("real or fake? {i} {j}");
        //             panic!();
        //         }

        //         swaps += 1;
        //     }
        // }

        let mut swappy = vec![self.one_z_swap().unwrap()];

        let original_terms = self.terms.clone();

        self.terms = original_terms.clone();
        self.remaining_terms = original_terms.clone();

        for i in 0..self.terms.len() {
            if z_term_indices.contains(&i) {
                continue;
            }

            for j in (i + 1)..self.terms.len() {
                if z_term_indices.contains(&j) {
                    continue;
                }

                self.terms = original_terms.clone();
                self.remaining_terms = original_terms.clone();

                self.wires.retain(|k, _| k.starts_with(['x', 'y']));

                self.swap_term_outputs(i, j);

                swappy.push((i, j));

                if let Some(swap) = self.one_z_swap() {
                    swappy.push(swap);

                    if let Some(swap) = self.one_z_swap() {
                        swappy.push(swap);

                        self.solve();

                        let at_least_there_was_an_attempt_sum = self.sum_bits('z');

                        if real_sum == at_least_there_was_an_attempt_sum {
                            eprintln!("real or fake? {i} {j} {:?}", &swappy);

                            for (from, to) in &swappy {
                                eprintln!("{}: {}", from, self.terms[*from].out);
                                eprintln!("{}: {}", to, self.terms[*to].out);
                            }
                            panic!();
                        }

                        swappy.pop();
                    }

                    swappy.pop();
                }

                swappy.pop();
            }
        }

        self.terms = original_terms.clone();
        self.remaining_terms = original_terms.clone();

        swappy.push(self.one_z_swap().unwrap());

        dbg!(&swappy);

        let original_terms = self.terms.clone();

        self.terms = original_terms.clone();
        self.remaining_terms = original_terms.clone();

        for i in 0..self.terms.len() {
            if z_term_indices.contains(&i) {
                continue;
            }

            for j in (i + 1)..self.terms.len() {
                if z_term_indices.contains(&j) {
                    continue;
                }

                self.terms = original_terms.clone();
                self.remaining_terms = original_terms.clone();

                self.wires.retain(|k, _| k.starts_with(['x', 'y']));

                self.swap_term_outputs(i, j);

                swappy.push((i, j));

                if let Some(swap) = self.one_z_swap() {
                    swappy.push(swap);

                    self.solve();

                    let at_least_there_was_an_attempt_sum = self.sum_bits('z');

                    if real_sum == at_least_there_was_an_attempt_sum {
                        eprintln!(
                            "real or fake? {i} {j} {} {} {:?}",
                            self.terms[i].out, self.terms[j].out, &swappy
                        );

                        for (from, to) in &swappy {
                            eprintln!("{}: {}", from, self.terms[*from].out);
                            eprintln!("{}: {}", to, self.terms[*to].out);
                        }
                        // panic!();
                    }

                    swappy.pop();
                }

                swappy.pop();
            }
        }

        dbg!(swappy);

        dbg!(swaps);
    }

    fn debug_dependencies(&self, fort: &'a str) {
        eprintln!("----");

        let for_term = self.terms.iter().find(|t| t.out == fort).unwrap();

        fn print(depth: usize, wirings: &Wirings, term: &Term) {
            let mut indent = depth * 2;

            eprintln!(
                "{:indent$}{} {} {} = {}",
                "",
                term.a,
                term.op.to_str(),
                term.b,
                term.out
            );

            indent += 2;

            if let Some(seed) = wirings.wires.get(term.a) {
                // eprintln!("{:indent$}{} = {seed}", "", term.a);
            } else {
                let term_a = wirings.terms.iter().find(|t| t.out == term.a).unwrap();

                print(depth + 1, wirings, term_a);
            }

            if let Some(seed) = wirings.wires.get(term.b) {
                // eprintln!("{:indent$}{} = {seed}", "", term.b);
            } else {
                let term_b = wirings.terms.iter().find(|t| t.out == term.b).unwrap();

                print(depth + 1, wirings, term_b);
            }
        }

        print(0, self, for_term);
    }

    fn reenact_divine_punishment(&self, swaps: &str) -> u64 {
        let mut subject = self.clone();

        let swaps = {
            let mut s = vec![];
            let mut lines = swaps.lines();

            while let Some(first) = lines.next() {
                let second = lines.next().unwrap();
                let first: usize = first.split_once(":").unwrap().0.parse().unwrap();
                let second: usize = second.split_once(":").unwrap().0.parse().unwrap();

                s.push((first, second));
            }

            s
        };

        for (i, j) in swaps {
            subject.swap_term_outputs(i, j);
        }

        subject.solve();

        subject.sum_bits('z')
    }
}

pub fn part1() {
    let mut wirings = Wirings::from_str(INPUT);

    wirings.solve();

    dbg!(wirings.sum_bits('z'));
}

pub fn part2() {
    let mut wirings = Wirings::from_str(INPUT);

    dbg!(wirings.add_seeds());

    let a = wirings.reenact_divine_punishment(
        "195: npf
14: z13
106: gws
158: nnt
207: cph
212: z19
101: hgj
90: z33",
    );

    dbg!(a);

    // wirings.shortcut_v2();

    // wirings.fix_fuck(&[], 0);

    // dbg!(wirings.add_seeds());

    // wirings.find_first_fucked_bit();

    // wirings.debug_dependencies("z00");
    // wirings.debug_dependencies("z01");
    // wirings.debug_dependencies("z02");
    // wirings.debug_dependencies("z03");
    // wirings.debug_dependencies("z04");
    // wirings.debug_dependencies("z05");
    // wirings.debug_dependencies("z06");
}

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

        wirings.debug_dependencies("z00");
        wirings.debug_dependencies("z01");
        wirings.debug_dependencies("z02");

        wirings.solve();
        dbg!(&wirings.terms);
        dbg!(wirings.sum_bits('z'));
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
        dbg!(wirings.sum_bits('z'));
        dbg!(wirings.add_seeds());
    }

    #[test]
    fn debug_long() {
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

        wirings.debug_dependencies("z00");
        wirings.debug_dependencies("z01");
        wirings.debug_dependencies("z02");
        wirings.debug_dependencies("z03");
        wirings.debug_dependencies("z04");
        wirings.debug_dependencies("z05");
    }

    #[test]
    fn debug_long_fixed() {
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

        wirings.debug_dependencies("z00");
        wirings.debug_dependencies("z01");
        wirings.debug_dependencies("z02");
        wirings.debug_dependencies("z03");
        wirings.debug_dependencies("z04");
        wirings.debug_dependencies("z05");
    }
}
