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
            (i + 1..n).filter_map(move |j| {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];
                let (min_x, max_x) = (x1.min(x2), x1.max(x2));
                let (min_y, max_y) = (y1.min(y2), y1.max(y2));

                points
                    .iter()
                    .all(|&(px, py)| px <= min_x || px >= max_x || py <= min_y || py >= max_y)
                    .then(|| (x1.abs_diff(x2) + 1) as usize * (y1.abs_diff(y2) + 1) as usize)
                    .inspect(|size| {
                        println!("({x1:2},{y1:2}) x ({x2:2},{y2:2}) = {}", size);
                    })
            })
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
