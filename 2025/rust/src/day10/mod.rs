#![allow(clippy::cast_possible_truncation)]
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

    fn fewest_presses(self) -> usize {
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
}

// fn get_presses(line: &str) -> u8 {
//     0
// }

fn get_joltage_presses(line: &str) -> u16 {
    let mut chunks = line.split_ascii_whitespace();
    let indicator_chunk = chunks.next().unwrap();
    let indicator_size = indicator_chunk.len() as u8 - 2;

    let joltage_chunk = chunks.next_back().unwrap();
    let joltage_target_state = joltage_chunk[1..joltage_chunk.len() - 1]
        .split(',')
        .map(|n| n.parse::<u16>().unwrap())
        .collect::<Vec<_>>();

    let button_sets: Vec<Vec<u8>> = chunks
        .map(|chunk| {
            let bs = chunk.as_bytes();
            bs[1..bs.len() - 1].chunks(2).map(|b| b[0] - b'0').collect()
        })
        .collect();
    let indicator_affectors: Vec<Vec<bool>> = (0..indicator_size)
        .map(|idx| button_sets.iter().map(|b| b.contains(&idx)).collect())
        .collect();

    let optimizer = Optimize::new();

    // b0, b1, b2, ... etc. "variables" in our linear equation set
    let linear_variables = (0..button_sets.len())
        .map(|i| Int::fresh_const(format!("b{i}").as_str()))
        .collect::<Vec<_>>();

    for (indicator_index, affectors) in indicator_affectors.iter().enumerate() {
        let eq_left = Int::sum(
            affectors
                .iter()
                .enumerate()
                .filter(|&(_, &affector)| affector)
                .map(|(i, _)| &linear_variables[i]),
        );
        optimizer.assert(&eq_left.eq(joltage_target_state[indicator_index]));
    }

    // b_n >= 0
    for lv in &linear_variables {
        optimizer.assert(&lv.ge(0));
    }
    // minimize(b0 + b1 + b2 + b3 + b4)
    let goal = linear_variables.iter().sum::<Int>();
    optimizer.minimize(&goal);

    // RUN the motherfucker
    match optimizer.check(&[]) {
        SatResult::Sat => {}
        _ => panic!(),
    }
    optimizer
        .get_model()
        .unwrap()
        .eval(&goal, true)
        .unwrap()
        .as_u64()
        .unwrap() as u16
}

#[tracing::instrument(skip_all)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 404
    // input.lines()
    input
        .lines()
        .map(Machine::parse)
        .map(Machine::fewest_presses)
        .sum()
}

#[tracing::instrument(skip_all)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = 16474
    input.par_lines().map(get_joltage_presses).sum::<u16>() as usize
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
