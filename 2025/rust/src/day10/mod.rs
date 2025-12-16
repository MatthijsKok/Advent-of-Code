#![allow(clippy::cast_possible_truncation)]
use std::collections::VecDeque;
use std::iter::Sum;

use rayon::prelude::*;
use z3::{Optimize, SatResult, ast::Int};

fn get_presses(line: &str) -> u16 {
    let mut chunks = line.split_ascii_whitespace();

    let indicator_chunk = chunks.next().unwrap().as_bytes();
    chunks.next_back(); // skip joltages
    let indicator_size = indicator_chunk.len() as u8 - 2;
    let indicator_target_state: u16 = indicator_chunk[1..indicator_chunk.len() - 1]
        .iter()
        .enumerate()
        .filter(|&(_, s)| *s == b'#')
        .fold(0, |acc, (i, _)| acc | (1 << i));

    let button_masks = chunks
        .map(|chunk| {
            let bytes = chunk.as_bytes();
            bytes[1..bytes.len() - 1]
                .chunks(2)
                .map(|b| b[0] - b'0')
                .filter(|i| *i < indicator_size)
                .fold(0, |acc, i| acc | 1 << i)
        })
        .collect::<Vec<_>>();

    let max_states = 1 << indicator_size;
    let mut dist = vec![u16::MAX; max_states];
    dist[0] = 0;

    let mut q: VecDeque<u16> = VecDeque::with_capacity(64);
    q.push_back(0);

    while let Some(state) = q.pop_front() {
        if state == indicator_target_state {
            return dist[state as usize];
        }
        let d = dist[state as usize];
        for &bm in &button_masks {
            let ns = state ^ bm;
            if dist[ns as usize] == u16::MAX {
                dist[ns as usize] = d + 1;
                q.push_back(ns);
            }
        }
    }

    unreachable!("target indicator state unreachable");
}

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
    input.lines().map(get_presses).sum::<u16>() as usize
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
