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
                .map(|(j, val)| ((i as isize, j as isize), val))
                .collect::<Vec<_>>()
        })
        .collect::<BTreeMap<_, _>>();
    for (&(i, j), &val) in &lookup {
        if val == '.' {
            continue;
        }
        let mut local_count: u8 = 0;
        for ni in i - 1..=i + 1 {
            if ni < 0 || ni >= size_i {
                continue;
            }
            for nj in j - 1..=j + 1 {
                if nj < 0 || nj >= size_j {
                    continue;
                }
                if i == ni && j == nj {
                    continue;
                }
                let nval = *(lookup.get(&(ni, nj)).unwrap());
                if nval == '@' {
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
    0
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
