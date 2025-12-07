use std::{array, panic};

use rayon::str::ParallelString;

#[tracing::instrument(skip_all, ret)]
pub(crate) fn solve_part1(input: &str) -> usize {
    // Answer = 5977759036837
    let input_width = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .count();
    assert_eq!(input_width, 1000);

    let mut data: [Vec<&str>; 1000] = array::repeat(Vec::new());

    for line in input.lines() {
        for (i, item) in line.split_ascii_whitespace().enumerate() {
            if let Some(data_item) = data.get_mut(i) {
                data_item.push(item);
            }
        }
    }

    data.iter_mut()
        .map(|item| -> usize {
            let sign = item.pop().unwrap();
            let numbers: Vec<usize> = item
                .iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            match sign {
                "*" => numbers.iter().product(),
                "+" => numbers.iter().sum(),
                _ => panic!(),
            }
        })
        .sum()
}

#[tracing::instrument(skip_all, ret)]
pub(crate) fn solve_part2(input: &str) -> usize {
    // Answer = ?
    0
}

#[test]
fn part1_examples() {}

#[test]
fn part2_examples() {}
