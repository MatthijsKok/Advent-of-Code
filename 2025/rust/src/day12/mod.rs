#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 536
    input
        .rsplit("\n\n")
        .next()
        .unwrap()
        .lines()
        .filter(|line| {
            // TODO: `.is_some_and()` for beautiful
            let (dims, presents) = line.split_once(": ").unwrap();
            let available_spots = dims
                .split_once('x')
                .map(|(d1, d2)| (d1.parse::<u16>().unwrap() / 3) * (d2.parse::<u16>().unwrap() / 3))
                .unwrap();
            let presents_sum: u16 = presents
                .split_ascii_whitespace()
                .map(|p| p.parse::<u16>().unwrap())
                .sum();

            available_spots >= presents_sum
        })
        .count()
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = ?
    input.lines().count();
    0
}

#[cfg(test)]
const EXAMPLE: &str = "\
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

#[test]
fn part1_examples() {
    assert_eq!(solve_part1(EXAMPLE), 2);
}

#[test]
fn part2_examples() {
    assert_eq!(solve_part2(EXAMPLE), 0);
}
