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

                // points
                //     .iter()
                //     .any(|&(px, py)| {
                //         px > x1.min(x2) && px < x1.max(x2) && py > y1.min(y2) && py < y1.max(y2)
                //     })
                //     .not()
                //     .then(|| (x1.abs_diff(x2) + 1) as usize * (y1.abs_diff(y2) + 1) as usize)

                // let has_point_inside = points.iter().any(|&(px, py)| {
                //     px > x1.min(x2) && px < x1.max(x2) && py > y1.min(y2) && py < y1.max(y2)
                // });
                // if has_point_inside {
                //     None
                // } else {
                //     Some((x1.abs_diff(x2) + 1) as usize * (y1.abs_diff(y2) + 1) as usize)
                // }
            })
        })
        .max()
        .unwrap()
}

#[test]
fn part1_examples() {}

#[test]
fn part2_examples() {}
