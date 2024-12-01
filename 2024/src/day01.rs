use std::time::Instant;

use tracing::{debug, info};

const LIST_LEN: usize = 1000;

#[tracing::instrument]
pub(crate) fn solve() {
    let input = include_str!("day01_input.txt");
    let (mut left, mut right) = parse_lists(input);
    left.sort();
    right.sort();

    part_1(&left, &right);
    part_2(&left, &right);
}

#[tracing::instrument(skip(input))]
fn parse_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
    let instant = Instant::now();
    let mut left = Vec::with_capacity(LIST_LEN);
    let mut right = Vec::with_capacity(LIST_LEN);

    for line in input.lines() {
        let mut split_iter = line.split_ascii_whitespace();
        left.push(split_iter.next().unwrap().parse::<i64>().unwrap());
        right.push(split_iter.next().unwrap().parse::<i64>().unwrap());
    }

    info!("Parsing lists took: {:?}", instant.elapsed());

    assert!(left.len() == LIST_LEN);
    assert!(left.len() == right.len());

    (left, right)
}

#[tracing::instrument(skip(left, right))]
fn part_1(left: &Vec<i64>, right: &Vec<i64>) {
    let instant_part1: Instant = Instant::now();
    let mut sum: i64 = 0;
    for i in 0..LIST_LEN {
        sum += (left[i] - right[i]).abs();
    }
    debug!("Part 1 took: {:?}", instant_part1.elapsed());
    info!("Part 1 answer: {}", sum);
}

#[tracing::instrument(skip(left, right))]
fn part_2(left: &Vec<i64>, right: &Vec<i64>) {
    let instant_part2 = Instant::now();
    let mut similarity: i64 = 0;
    for i in left.iter() {
        for j in right.iter() {
            match i.cmp(j) {
                std::cmp::Ordering::Equal => {
                    similarity += i;
                }
                std::cmp::Ordering::Less => break,
                std::cmp::Ordering::Greater => continue,
            }
        }
    }
    debug!("Part 2 took: {:?}", instant_part2.elapsed());
    info!("Part 2 answer: {}", similarity);
}
