use std::time::Instant;

const LIST_LEN: usize = 1000;

pub(crate) fn solve() {
    let input = include_str!("day01_input.txt");
    let (mut left, mut right) = parse_lists(input);
    left.sort();
    right.sort();

    let instant_part1 = Instant::now();
    let mut sum: i64 = 0;
    for i in 0..LIST_LEN {
        sum += (left[i] - right[i]).abs();
    }
    println!("Part 1 took: {:?}", instant_part1.elapsed());

    let instant_part2 = Instant::now();
    let mut similarity: i64 = 0;
    for i in left.iter() {
        for j in right.iter() {
            if i == j {
                similarity += i;
            } else if i < j {
                break;
            }
        }
    }
    println!("Part 2 took: {:?}", instant_part2.elapsed());

    println!("Day 1 Part 1: {}", sum);
    println!("Day 1 Part 2: {}", similarity);
}

fn parse_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
    let instant = Instant::now();
    let mut left = Vec::with_capacity(LIST_LEN);
    let mut right = Vec::with_capacity(LIST_LEN);

    for line in input.lines() {
        let mut split_iter = line.split_ascii_whitespace();
        left.push(split_iter.next().unwrap().parse::<i64>().unwrap());
        right.push(split_iter.next().unwrap().parse::<i64>().unwrap());
    }

    println!("Parsing lists took: {:?}", instant.elapsed());

    assert!(left.len() == LIST_LEN);
    assert!(left.len() == right.len());

    (left, right)
}
