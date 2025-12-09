use itertools::Itertools;

#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 4725826296
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        })
        .array_combinations()
        .map(|[(x1, y1), (x2, y2)]| (x1.abs_diff(x2) + 1) as usize * (y1.abs_diff(y2) + 1) as usize)
        .max()
        .unwrap()
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = ?
    0
}

#[test]
fn part1_examples() {}

#[test]
fn part2_examples() {}
