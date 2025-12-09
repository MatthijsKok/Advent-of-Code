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
    // Answer = 1637556834
    let points_vec = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            (l.parse::<u32>().unwrap(), r.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();
    let points = points_vec.as_slice();
    let n = points.len();

    // connect all the points (WRAPPING) to their neighbours
    let edges_vec = (0..n)
        .map(|i| (points[i], points[(i + 1) % n]))
        .collect::<Vec<_>>();
    let edges = edges_vec.as_slice();

    (0..n)
        .flat_map(|i| {
            (i + 1..n).filter_map(move |j| {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];
                let (min_x, max_x) = (x1.min(x2), x1.max(x2));
                let (min_y, max_y) = (y1.min(y2), y1.max(y2));

                // All edges are outside of the rectangle
                let allowed = edges.iter().all(|&((ex1, ey1), (ex2, ey2))| {
                    if ey1 == ey2 {
                        // Horizontal edge
                        // let (min_ex, max_ex) = (ex1.min(ex2), ex1.max(ex2));
                        // ey1 <= min_y || ey1 >= max_y || // completely different row
                        // ex1.min(ex2) >= max_x || // both edge extremes are larger
                        // ex1.max(ex2) <= min_x // both edge extremes are smaller
                        // let (min_ex, max_ex) = (ex1.min(ex2), ex1.max(ex2));
                        // ey1 <= min_y || ey1 >= max_y || min_ex >= max_x || max_ex <= min_x
                        ey1 <= min_y
                            || ey1 >= max_y
                            || ex1.min(ex2) >= max_x
                            || ex1.max(ex2) <= min_x
                    } else {
                        // Vertical edge
                        // let (min_ey, max_ey) = (ey1.min(ey2), ey1.max(ey2));
                        // ex1 <= min_x || ex1 >= max_x || min_ey >= max_y || max_ey <= min_y
                        ex1 <= min_x
                            || ex1 >= max_x
                            || ey1.min(ey2) >= max_y
                            || ey1.max(ey2) <= min_y
                    }
                });

                // let cx = f64::from(min_x + max_x) / 2.0;
                // let cy = f64::from(min_y + max_y) / 2.0;
                // let crossings = edges
                //     .iter()
                //     .filter(|&&((ex1, ey1), (ex2, ey2))| {
                //         ex1 == ex2
                //             && f64::from(ex1) > cx
                //             && cy > f64::from(ey1.min(ey2))
                //             && cy < f64::from(ey1.max(ey2))
                //     })
                //     .count();

                // (allowed && crossings % 2 == 1)
                //     .then(|| (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize)

                allowed.then(|| (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize)

                // points
                //     .iter()
                //     .all(|&(px, py)| px <= min_x || px >= max_x || py <= min_y || py >= max_y)
                //     .then(|| (x1.abs_diff(x2) + 1) as usize * (y1.abs_diff(y2) + 1) as usize)
                //     .inspect(|size| {
                //         println!("({x1:2},{y1:2}) x ({x2:2},{y2:2}) = {}", size);
                //     })
            })
        })
        .max()
        .unwrap()

    // let mut max_area = 0;

    // for i in 0..n {
    //     for j in i + 1..n {
    //         let (x1, y1) = points[i];
    //         let (x2, y2) = points[j];
    //         let (min_x, max_x) = (x1.min(x2), x1.max(x2));
    //         let (min_y, max_y) = (y1.min(y2), y1.max(y2));

    //         let blocked = edges.iter().any(|&((ex1, ey1), (ex2, ey2))| {
    //             if ey1 == ey2 {
    //                 ey1 > min_y && ey1 < max_y && ex1.min(ex2) < max_x && ex1.max(ex2) > min_x
    //             } else {
    //                 ex1 > min_x && ex1 < max_x && ey1.min(ey2) < max_y && ey1.max(ey2) > min_y
    //             }
    //         });
    //         if blocked {
    //             continue;
    //         }

    //         // ray casting
    //         let cx = (min_x + max_x) as f64 / 2.0;
    //         let cy = (min_y + max_y) as f64 / 2.0;
    //         let crossings = edges
    //             .iter()
    //             .filter(|&&((ex1, ey1), (ex2, ey2))| {
    //                 ex1 == ex2
    //                     && (ex1 as f64) > cx
    //                     && cy > ey1.min(ey2) as f64
    //                     && cy < ey1.max(ey2) as f64
    //             })
    //             .count();

    //         if crossings % 2 == 1 {
    //             max_area =
    //                 max_area.max((max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize);
    //         }
    //     }
    // }

    // max_area

    // (0..n)
    //     .flat_map(|i| {
    //         (i + 1..n).filter_map(move |j| {
    //             let (x1, y1) = points[i];
    //             let (x2, y2) = points[j];
    //             let (min_x, max_x) = (x1.min(x2), x1.max(x2));
    //             let (min_y, max_y) = (y1.min(y2), y1.max(y2));

    //             points
    //                 .iter()
    //                 .all(|&(px, py)| px <= min_x || px >= max_x || py <= min_y || py >= max_y)
    //                 .then(|| (x1.abs_diff(x2) + 1) as usize * (y1.abs_diff(y2) + 1) as usize)
    //                 .inspect(|size| {
    //                     println!("({x1:2},{y1:2}) x ({x2:2},{y2:2}) = {}", size);
    //                 })
    //         })
    //     })
    //     .max()
    //     .unwrap()
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
