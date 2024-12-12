const LIST_LEN: usize = 1_000;

#[tracing::instrument(skip_all, ret)]
pub fn solve_day01(input: &str) -> (u32, u32) {
    let (left, right) = parse_input(input);
    let part1 = solve_part_1(&left, &right);
    let part2 = solve_part_2(&left, &right);
    (part1, part2)
}

#[tracing::instrument(skip_all)]
fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::with_capacity(LIST_LEN);
    let mut right: Vec<u32> = Vec::with_capacity(LIST_LEN);

    for line in input.lines() {
        left.push(line[..5].parse::<u32>().unwrap());
        right.push(line[8..].parse::<u32>().unwrap());
    }

    left.sort_unstable();
    right.sort_unstable();

    debug_assert_eq!(LIST_LEN, left.len());
    debug_assert_eq!(LIST_LEN, right.len());

    (left, right)
}

#[tracing::instrument(skip_all)]
fn solve_part_1(left: &[u32], right: &[u32]) -> u32 {
    left.iter()
        .zip(right.iter())
        .fold(0, |acc, (&l, &r)| acc + l.abs_diff(r))
}

#[tracing::instrument(skip_all)]
fn solve_part_2(left: &[u32], right: &[u32]) -> u32 {
    let mut similarity: u32 = 0;
    let mut lookup: Vec<u32> = vec![0; 100_000];
    for i in right.iter() {
        lookup[*i as usize] += *i;
    }
    for i in left.iter() {
        similarity += lookup[*i as usize];
    }
    similarity
}
