#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 4725826296
    let points = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();
    let n = points.len();
    (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
        .map(|(i, j)| {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            (x1.abs_diff(x2) + 1) as usize * (y1.abs_diff(y2) + 1) as usize
        })
        .max()
        .unwrap()
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = 1637556834
    let points = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();
    let n = points.len();

    // connect all the points (WRAPPING) to their neighbours
    let mut edges = (0..n)
        .map(|i| (points[i], points[(i + 1) % n]))
        .collect::<Vec<_>>();
    edges.sort_unstable();

    (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
        .filter_map(|(i, j)| {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let (min_x, max_x) = (x1.min(x2), x1.max(x2));
            let (min_y, max_y) = (y1.min(y2), y1.max(y2));
            edges
                .iter()
                .all(|&((ex1, ey1), (ex2, ey2))| {
                    if ey1 == ey2 {
                        ey1 <= min_y
                            || ey1 >= max_y
                            || ex1.min(ex2) >= max_x
                            || ex1.max(ex2) <= min_x
                    } else {
                        ex1 <= min_x
                            || ex1 >= max_x
                            || ey1.min(ey2) >= max_y
                            || ey1.max(ey2) <= min_y
                    }
                })
                .then(|| (x1.abs_diff(x2) + 1) as usize * (y1.abs_diff(y2) + 1) as usize)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
/// ```
///  01234567890123
/// 0..............
/// 1.......#...#..
/// 2..............
/// 3..#....#......
/// 4..............
/// 5..#......#....
/// 6..............
/// 7.........#.#..
/// 8..............
/// ```
const EXAMPLE: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

#[test]
fn part1_examples() {
    assert_eq!(solve_part1(EXAMPLE), 50);
}

#[test]
fn part2_examples() {
    assert_eq!(solve_part2(EXAMPLE), 24);
}
