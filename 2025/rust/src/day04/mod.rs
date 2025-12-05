use std::collections::{BTreeMap, HashMap};

use rayon::prelude::*;

#[tracing::instrument(skip_all, ret)]
pub(crate) fn solve_part1(input: &str) -> usize {
    // Answer = 1474
    let mut count: usize = 0;

    let size_i = input.lines().count() as isize;
    let size_j = input.lines().next().unwrap().len() as isize;

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
        for ni in 0.max(i - 1)..=(size_i - 1).min(i + 1) {
            for nj in 0.max(j - 1)..=(size_j - 1).min(j + 1) {
                if i == ni && j == nj {
                    continue;
                }
                let nval = *(lookup.get(&(ni, nj)).unwrap());
                if nval == 1 {
                    local_count += 1;
                }
            }
        }
        if local_count < 4 {
            count += 1;
        }
    }
    count
}

#[tracing::instrument(skip_all, ret)]
pub(crate) fn solve_part2(input: &str) -> usize {
    // Answer = ?
    let mut count: usize = 0;

    let size_i = input.lines().count() as isize;
    let size_j = input.lines().next().unwrap().len() as isize;

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
        for ni in 0.max(i - 1)..=(size_i - 1).min(i + 1) {
            for nj in 0.max(j - 1)..=(size_j - 1).min(j + 1) {
                if i == ni && j == nj {
                    continue;
                }
                let nval = *(lookup.get(&(ni, nj)).unwrap());
                local_count += nval;
            }
        }
        if local_count < 4 {
            count += 1;
        }
    }
    count

    // let a = input
    //     .lines()
    //     .enumerate()
    //     .flat_map(|(i, line)| {
    //         line.chars()
    //             .enumerate()
    //             .map(|(j, val)| ((i as isize, j as isize), val))
    //             .collect::<Vec<_>>()
    //     })
    //     .filter(|(coords, val)| *val != '.')
    //     .map(|((i, j), val)| {
    //         //
    //     });
    // 0
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
