const LIST_LEN: usize = 1000;

pub(crate) fn solve() {
    let input = include_str!("day01_input.txt");
    let (mut left, mut right) = parse_lists(input);
    left.sort();
    right.sort();
    let mut deltas: Vec<i64> = Vec::new();
    for (left_item, right_item) in left.iter().zip(right.iter()) {
        deltas.push((left_item - right_item).abs());
    }
    let sum: i64 = deltas.iter().sum();
    println!("Day 1: Part 1 Solution: {}", sum);

    // naive O(n^2) solution
    let mut similarity: i64 = 0;
    for i in left.iter() {
        for j in right.iter() {
            if i == j {
                similarity += i;
            }
        }
    }
    println!("Day 1: Part 2 Solution: {}", similarity);
}

fn parse_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut split_iter = line.split_ascii_whitespace();
        left.push(split_iter.next().unwrap().parse::<i64>().unwrap());
        right.push(split_iter.next().unwrap().parse::<i64>().unwrap());
    }

    assert!(left.len() == LIST_LEN);
    assert!(left.len() == right.len());

    (left, right)
}
