use std::collections::{BTreeMap, HashMap};

use rayon::prelude::*;

// Assignment states 4 neighbours as threshold. If we include yourself,
// we do not need to branch inside the hot loop.
const NEIGHBOUR_THRESHOLD: u8 = 5;

#[tracing::instrument(skip_all, ret)]
pub(crate) fn solve_part1(input: &str) -> usize {
    // Answer = 1474
    let mut count: usize = 0;
    let dim_size = input.lines().next().unwrap().len() as isize;

    let lookup = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, val)| ((i as isize, j as isize), (val == '@') as u8))
                .collect::<Vec<_>>()
        })
        .collect::<BTreeMap<_, _>>();
    for (&(i, j), &val) in &lookup {
        if val == 0 {
            continue;
        }
        let mut local_count: u8 = 0;
        for ni in 0.max(i - 1)..=(dim_size - 1).min(i + 1) {
            for nj in 0.max(j - 1)..=(dim_size - 1).min(j + 1) {
                // NOTE: Don't need to do check, just bumped Neighbour Threshold by 1
                // if i == ni && j == nj {
                //     continue;
                // }
                local_count += *(lookup.get(&(ni, nj)).unwrap());
            }
        }
        if local_count < NEIGHBOUR_THRESHOLD {
            count += 1;
        }
    }
    count
}

#[tracing::instrument(skip_all, ret)]
pub(crate) fn solve_part2(input: &str) -> usize {
    // Answer = ?
    let mut count: usize = 0;

    let dim_size = input.lines().next().unwrap().len() as isize;

    let lookup: BTreeMap<(isize, isize), u8> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, val)| ((i as isize, j as isize), (val == '@') as u8))
                .collect::<Vec<_>>()
        })
        .collect();
    lookup
        .iter()
        .filter(|&(_, &v)| v != 0)
        .map(|(&(i, j), _)| {
            let mut neighbours_count: u8 = 0;
            for ni in 0.max(i - 1)..=(dim_size - 1).min(i + 1) {
                for nj in 0.max(j - 1)..=(dim_size - 1).min(j + 1) {
                    // NOTE: Don't need to do check, just bump Neighbour Threshold by 1
                    // if i == ni && j == nj {
                    //     continue;
                    // }
                    neighbours_count += *(lookup.get(&(ni, nj)).unwrap());
                }
            }
            neighbours_count
        })
        // Neighbour Count Threshold
        // .filter(|&n| n < 4)
        .filter(|&n| n < 5)
        .count()
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
fn part2_examples() {
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
    assert_eq!(solve_part2(&example), 13);
}
