use std::collections::VecDeque;
use std::fmt::Debug;
use std::iter::Sum;

use rayon::prelude::*;
use z3::{Optimize, SatResult, ast::Int};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum IndicatorState {
    Off,
    On,
}

// impl std::ops::Not for IndicatorState {
//     type Output = Self;
//     fn not(self) -> Self::Output {
//         match self {
//             Self::Off => Self::On,
//             Self::On => Self::Off,
//         }
//     }
// }

impl TryFrom<char> for IndicatorState {
    type Error = char;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::On),
            '.' => Ok(Self::Off),
            _ => Err(value),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
struct Machine {
    indicator_size: usize,
    indicator_target_state: usize,
    button_sets: Vec<Vec<u8>>,
    joltage_target_state: Vec<u16>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let mut chunks = line.split_ascii_whitespace();
        let indicators = chunks
            .next()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .chars()
            .map(|c| c.try_into().unwrap())
            .collect::<Vec<IndicatorState>>();
        let joltage_target_state = chunks
            .next_back()
            .unwrap()
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(|s| s.parse::<u16>().unwrap())
            .collect::<Vec<_>>();

        let button_sets: Vec<Vec<u8>> = chunks
            .map(|chunk| {
                chunk
                    .chars()
                    .filter_map(|c| c.to_digit(10).and_then(|d| u8::try_from(d).ok()))
                    .collect()
            })
            .collect();
        // button_sets.sort_by_key(Vec::len);
        // button_sets.reverse();

        let indicator_size = indicators.len();
        let indicator_target_state = indicators.iter().enumerate().fold(0usize, |acc, (i, s)| {
            if *s == IndicatorState::On {
                acc | (1usize << i)
            } else {
                acc
            }
        });

        Self {
            indicator_size,
            indicator_target_state,
            button_sets,
            joltage_target_state,
        }
    }

    fn fewest_presses(&self) -> usize {
        let mut button_masks: Vec<usize> = Vec::with_capacity(self.button_sets.len());
        for button in &self.button_sets {
            let mut mask = 0usize;
            for &i in button {
                if i as usize >= self.indicator_size {
                    continue;
                }
                mask |= 1usize << i;
            }
            button_masks.push(mask);
        }

        let max_states = 1usize << self.indicator_size;
        let mut dist = vec![usize::MAX; max_states];
        let mut q = VecDeque::with_capacity(64);

        dist[0] = 0;
        q.push_back(0usize);

        while let Some(state) = q.pop_front() {
            if state == self.indicator_target_state {
                return dist[state];
            }
            let d = dist[state];
            for &bm in &button_masks {
                let ns = state ^ bm;
                if dist[ns] == usize::MAX {
                    dist[ns] = d + 1;
                    q.push_back(ns);
                }
            }
        }

        unreachable!("target indicator state unreachable");
    }

    // #[tracing::instrument(skip_all)]
    fn fewest_presses_joltage(&self) -> usize {
        // https://docs.rs/z3/latest/z3/

        // [##...##] (0,3,4,6) (1,2,4,5,6) (0,1,2,5,6) (0,1,3,5) (0,2,3,4,6) {29,26,26,12,9,26,26}
        let indicator_affectors: Vec<Vec<bool>> = (0..u8::try_from(self.indicator_size).unwrap())
            .map(|idx| self.button_sets.iter().map(|b| b.contains(&idx)).collect())
            .collect();
        // println!("{indicator_affectors:?}");
        // [[true,  false, true,  true,  true],  AKA "indicator 0 is in buttons 0,2,3,4"
        //  [false, true,  true,  true,  false],
        //  [false, true,  true,  false, true],
        //  [true,  false, false, true,  true],
        //  [true,  true,  false, false, true],
        //  [false, true,  true,  true,  false],
        //  [true,  true,  true,  false, true]]

        // The system of linear equations we need to solve is:
        // b0 +      b2 + b3 + b4 = 29
        //      b1 + b2 + b3      = 26
        //      b1 + b2 +      b4 = 26
        // b0 +           b3 + b4 = 12
        // b0 + b1 +           b4 = 9
        //      b1 + b2 + b3      = 26
        // b0 + b1 + b2 +      b4 = 26
        // And then the answer is "for which solution is smallest: b0+b1+b2+b3+b4 ?"

        let optimizer = Optimize::new();

        // b0, b1, b2, ... etc. "variables" in our linear equation set
        let linear_variables = (0..self.button_sets.len())
            .map(|i| Int::fresh_const(format!("b{i}").as_str()))
            .collect::<Vec<_>>();

        for lin_var in &linear_variables {
            optimizer.assert(&lin_var.ge(0));
        }

        for (indicator_index, affectors) in indicator_affectors.iter().enumerate() {
            let eq_left = Int::sum(
                affectors
                    .iter()
                    .enumerate()
                    .filter(|&(_, &affector)| affector)
                    .map(|(i, _)| &linear_variables[i]),
            );
            optimizer.assert(&eq_left.eq(self.joltage_target_state[indicator_index]));
        }

        // minimize(b0 + b1 + b2 + b3 + b4)
        let goal = linear_variables.iter().sum::<Int>();
        optimizer.minimize(&goal);

        // RUN the motherfucker
        match optimizer.check(&[]) {
            SatResult::Sat => {}
            _ => panic!(),
        }
        // dbg!(&optimizer);
        // dbg!(&optimizer.get_model());
        // dbg!(&optimizer.get_objectives());
        // dbg!(&optimizer.get_statistics());

        let optimizer_model = optimizer.get_model().unwrap();
        // dbg!(&optimizer_model);
        // b0!0 -> 0
        // b1!1 -> 3
        // b2!2 -> 17
        // b3!3 -> 6
        // b4!4 -> 6

        // dbg!(optimizer_model.eval(&goal, true).unwrap());
        // 32

        // AKA
        // (0,3,4,6)    0 times
        // (1,2,4,5,6)  3 times
        // (0,1,2,5,6)  17 times
        // (0,1,3,5)    6 times
        // (0,2,3,4,6)  6 times

        // Which would result in indicator values:
        // 0: 0 + 17 + 6 + 6 = 29
        // 1: 3 + 17 + 6 = 26
        // ... etc

        // which matches the joltage values
        // {29,26,26,12,9,26,26}

        optimizer_model
            .eval(&goal, true)
            .unwrap()
            .as_u64()
            .unwrap()
            .try_into()
            .unwrap()
    }
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Machine")
            .field(
                "indicator_size",
                &format_args!("{:?}", &self.indicator_size),
            )
            .field(
                "indicator_target_state",
                &format_args!("{:?}", &self.indicator_target_state),
            )
            .field("button_sets", &format_args!("{:?}", &self.button_sets))
            .field(
                "joltage_target_state",
                &format_args!("{:?}", &self.joltage_target_state),
            )
            .finish()
    }
}

#[tracing::instrument(skip_all)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 404
    // NOTES:
    // There can be at most 10 indicators
    // -> indicator index is a single digit number
    let machines = input.lines().map(Machine::parse).collect::<Vec<_>>();
    // dbg!(&machines[0..2]);
    machines.iter().map(Machine::fewest_presses).sum()
}

#[tracing::instrument(skip_all)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = 16474
    let machines = input.lines().map(Machine::parse).collect::<Vec<_>>();
    machines
        .par_iter()
        // .take(1)
        .map(Machine::fewest_presses_joltage)
        .sum()
}

#[cfg(test)]
const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

#[test]
fn part1_examples() {
    let line1 = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
    assert_eq!(solve_part1(line1), 2);
    let line2 = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
    assert_eq!(solve_part1(line2), 3);
    let line3 = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    assert_eq!(solve_part1(line3), 2);
    assert_eq!(solve_part1(EXAMPLE), 7);
}

#[test]
fn part2_examples() {
    let line1 = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
    assert_eq!(solve_part2(line1), 10);
    let line2 = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
    assert_eq!(solve_part2(line2), 12);
    let line3 = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    assert_eq!(solve_part2(line3), 11);
    assert_eq!(solve_part2(EXAMPLE), 33);
}
