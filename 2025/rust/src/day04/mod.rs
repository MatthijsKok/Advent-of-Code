use std::collections::{BTreeMap, HashSet};

use rayon::prelude::*;

// Assignment states 4 neighbours as threshold. If we include yourself,
// we do not need to branch inside the hot loop.
const NEIGHBOUR_THRESHOLD: u8 = 5;

fn neighbours(i: usize, j: usize, upper_bound: usize) -> impl Iterator<Item = (usize, usize)> {
    let start_i = i.saturating_sub(1);
    let start_j = j.saturating_sub(1);
    let end_i = (upper_bound - 1).min(i + 1);
    let end_j = (upper_bound - 1).min(j + 1);
    (start_i..=end_i).flat_map(move |ni| (start_j..=end_j).map(move |nj| (ni, nj)))
}

#[tracing::instrument(skip_all)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 1474
    let dim_size = input.lines().next().unwrap().len();

    let lookup = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.bytes()
                .enumerate()
                .map(|(j, val)| ((i, j), u8::from(val == b'@')))
                .collect::<Vec<_>>()
        })
        .collect::<BTreeMap<_, _>>();

    lookup
        .par_iter()
        .filter(|&(_, &v)| v != 0)
        .map(|(&(i, j), _)| -> u8 {
            neighbours(i, j, dim_size)
                .filter_map(|coord| lookup.get(&coord))
                .sum()
        })
        .filter(|&n| n < NEIGHBOUR_THRESHOLD)
        .count()
}

#[tracing::instrument(skip_all)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = 8910
    let mut count: usize = 0;
    let dim_size = input.lines().next().unwrap().len();

    let mut lookup = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.bytes()
                .enumerate()
                .filter_map(move |(j, val)| (val == b'@').then_some((i, j)))
        })
        .collect::<HashSet<_>>();

    loop {
        let new_removals = lookup
            .par_iter()
            .copied()
            .filter(|&(i, j)| {
                let n = neighbours(i, j, dim_size)
                    .filter(|coord| lookup.contains(coord))
                    .count();
                n < NEIGHBOUR_THRESHOLD as usize
            })
            .collect::<Vec<_>>();
        if new_removals.is_empty() {
            break;
        }
        count += new_removals.len();
        for coord in &new_removals {
            lookup.remove(coord);
        }
    }

    count
}

#[test]
fn part1_examples() {
    let example = [
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ]
    .join("\n");
    assert_eq!(solve_part1(&example), 13);
}

#[test]
fn part2_examples() {}
