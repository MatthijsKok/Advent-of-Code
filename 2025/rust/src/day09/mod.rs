#[tracing::instrument(skip_all)]
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

#[tracing::instrument(skip_all)]
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
    // partition edges into horizontal and vertical, to save this check inside the hot loop
    let (edges_horizontal, edges_vertical): (Vec<_>, Vec<_>) = (0..n)
        .map(|i| (points[i], points[(i + 1) % n]))
        .partition(|&((_, y1), (_, y2))| y1 == y2);

    let mut edges_horizontal_mapped = edges_horizontal
        .into_iter()
        .map(|((x1, y1), (x2, _))| (y1, x1.min(x2), x1.max(x2)))
        .collect::<Vec<_>>();
    edges_horizontal_mapped.sort_unstable_by_key(|&(y, _, _)| y);
    let mut edges_vertical_mapped = edges_vertical
        .into_iter()
        .map(|((x1, y1), (_, y2))| (x1, y1.min(y2), y1.max(y2)))
        .collect::<Vec<_>>();
    edges_vertical_mapped.sort_unstable_by_key(|&(x, _, _)| x);

    let mut max_area = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let (min_x, max_x) = (x1.min(x2), x1.max(x2));
            let (min_y, max_y) = (y1.min(y2), y1.max(y2));
            let area = (x1.abs_diff(x2) + 1) as usize * (y1.abs_diff(y2) + 1) as usize;

            // skip expensive comparison with all edges if the area is smaller anyway
            if area > max_area
                && edges_horizontal_mapped.iter().all(|&(ey, ex_min, ex_max)| {
                    ey <= min_y || ey >= max_y || ex_min >= max_x || ex_max <= min_x
                })
                && edges_vertical_mapped.iter().all(|&(ex, ey_min, ey_max)| {
                    ex <= min_x || ex >= max_x || ey_min >= max_y || ey_max <= min_y
                })
            {
                max_area = area;
            }
        }
    }
    max_area
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
