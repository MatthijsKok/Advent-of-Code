use std::{cmp::PartialOrd, ops::RangeInclusive};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn merge_all_ranges(mut ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    ranges.sort_by_key(|range| *range.start());
    ranges.into_iter().fold(Vec::new(), |mut merged, range| {
        match merged.last_mut() {
            Some(last) if *last.end() + 1 >= *range.start() => {
                *last = *last.start()..=(*last.end().max(range.end()))
            }
            _ => merged.push(range),
        }
        merged
    })
}

#[tracing::instrument(skip_all, ret)]
pub(crate) fn solve_part1(input: &str) -> usize {
    // Answer = 739
    let mut ranges: Vec<RangeInclusive<usize>> = Vec::new();
    let mut ingredients: Vec<usize> = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        ranges.push(
            line.split_once('-')
                .map(|(lb, ub)| (lb.parse::<usize>().unwrap(), ub.parse::<usize>().unwrap()))
                .map(|(lb, ub)| lb..=ub)
                .unwrap(),
        )
    }
    while let Some(line) = lines.next() {
        ingredients.push(line.parse().unwrap())
    }

    let ranges = merge_all_ranges(ranges);

    ingredients
        .par_iter()
        .filter(|&ingredient| ranges.iter().any(|r| r.contains(ingredient)))
        .count()
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
