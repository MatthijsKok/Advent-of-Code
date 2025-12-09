#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 4725826296
    let points_vec = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();
    let points = points_vec.as_slice();
    let n = points.len();
    (0..n)
        .flat_map(|i| {
            (i + 1..n).map(move |j| {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];
                (x1.abs_diff(x2) + 1) as usize * (y1.abs_diff(y2) + 1) as usize
            })
        })
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
