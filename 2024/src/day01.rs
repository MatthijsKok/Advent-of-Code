use std::time::Instant;

use tracing::{debug, info};

const LIST_LEN: usize = 1_000;

#[tracing::instrument]
pub fn solve() {
    let input = include_str!("day01_input.txt");
    let (mut left, mut right) = parse_lists(input);
    left.sort();
    right.sort();

    part_1(&left, &right);
    part_2(&left, &right);
}

#[tracing::instrument(skip(input))]
pub fn parse_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let instant = Instant::now();
    let mut left: Vec<u32> = Vec::with_capacity(LIST_LEN);
    let mut right: Vec<u32> = Vec::with_capacity(LIST_LEN);

    for line in input.lines() {
        left.push(line[..5].parse::<u32>().unwrap());
        right.push(line[8..].parse::<u32>().unwrap());
    }

    info!("Parsing lists took: {:?}", instant.elapsed());

    assert!(left.len() == LIST_LEN);
    assert!(left.len() == right.len());

    (left, right)
}

#[tracing::instrument(skip(left, right))]
pub fn part_1(left: &[u32], right: &[u32]) {
    let instant_part1: Instant = Instant::now();
    let mut sum: i64 = 0;
    for i in 0..LIST_LEN {
        sum += (left[i] as i64 - right[i] as i64).abs();
    }
    debug!("Part 1 took: {:?}", instant_part1.elapsed());
    info!("Part 1 answer: {}", sum);
}

#[tracing::instrument(skip(left, right))]
pub fn part_2(left: &[u32], right: &[u32]) {
    let instant_part2 = Instant::now();
    let mut similarity: u32 = 0;
    let mut lookup: Vec<u32> = vec![0; 100_000];
    for i in right.iter() {
        lookup[*i as usize] += *i;
    }
    for i in left.iter() {
        similarity += lookup[*i as usize];
    }
    debug!("Part 2 took: {:?}", instant_part2.elapsed());
    info!("Part 2 answer: {}", similarity);
}
