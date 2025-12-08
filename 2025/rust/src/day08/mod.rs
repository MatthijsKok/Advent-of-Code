// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
// struct Point(u32, u32, u32);

// impl Point {
//     fn dist(self, other: &Self) -> f64 {
//         let dx = u64::pow(self.0.abs_diff(other.0).into(), 2);
//         let dy = u64::pow(self.1.abs_diff(other.1).into(), 2);
//         let dz = u64::pow(self.2.abs_diff(other.2).into(), 2);
//         f64::sqrt((dx + dy + dz) as f64)
//     }
// }

#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> usize {
    // println!("{}", Point(0, 0, 0).dist(&Point(100000, 0, 0)));
    // println!("{}", Point(0, 0, 0).dist(&Point(100_000, 100_000, 0)));
    // println!("{}", Point(0, 0, 0).dist(&Point(100_000, 100_000, 100_000)));
    // Answer = ?
    0
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
