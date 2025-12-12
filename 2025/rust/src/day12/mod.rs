struct Present {
    index: u8,
    shape: [[bool; 3]; 3],
}

struct Region {
    dimensions: (u8, u8),
    desired_presents: [u8; 5],
}

fn parse_input(input: &str) -> (Vec<Present>, Vec<Region>) {
    let mut presents = vec![];
    let mut regions = vec![];

    let mut lines = input.lines();
    while let Some(line) = lines.next()
        && !line.contains('x')
    {}

    for line in lines {
        // let (dims, prs) = line.split_once(": ").unwrap();
        // let dimensions = dims.split_once('x').unwrap();
        // prs
        line.split_once(": ").map(|(dims, ps)| {
            Region {
                dimensions: dims
                    .split_once('x')
                    .map(|(d1, d2)| (d1.parse().unwrap(), d2.parse().unwrap()))
                    .unwrap(),
                desired_presents: ps
                    .split_ascii_whitespace()
                    .map(|p| p.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            }
            //
        });
    }

    (presents, regions)
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = ?
    input.lines().count();
    0
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = ?
    input.lines().count();
    0
}

#[cfg(test)]
const EXAMPLE: &str = "\
";

#[test]
fn part1_examples() {
    assert_eq!(solve_part1(EXAMPLE), 0);
}

#[test]
fn part2_examples() {
    assert_eq!(solve_part2(EXAMPLE), 0);
}
