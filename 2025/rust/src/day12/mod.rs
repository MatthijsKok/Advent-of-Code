#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Present {
    index: u8,
    shape: [[bool; 3]; 3],
}

fn parse_present(input: &[&str]) -> Present {
    Present {
        index: input[0][0..1].parse().unwrap(),
        shape: input[1..]
            .iter()
            .map(|line| {
                line.as_bytes()
                    .iter()
                    .map(|b| *b == b'#')
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Region {
    dimensions: (u16, u16),
    desired_presents: [u16; 6],
    // total_desired: u8,
}

fn parse_region(input: &str) -> Region {
    input
        .split_once(": ")
        .map(|(dims, ps)| Region {
            dimensions: dims
                .split_once('x')
                .map(|(d1, d2)| (d1.parse().unwrap(), d2.parse().unwrap()))
                .unwrap(),
            desired_presents: ps
                .split_ascii_whitespace()
                .map(|p| p.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            // total_desired: ps
            //     .split_ascii_whitespace()
            //     .map(|p| p.parse::<u8>().unwrap())
            //     .sum(),
        })
        .unwrap()
}

fn parse_input(input: &str) -> (Vec<Present>, Vec<Region>) {
    let lines = input.lines().collect::<Vec<_>>();
    let mut chunks = lines.split(|line| line.is_empty());

    let regions: Vec<Region> = chunks
        .next_back()
        .unwrap()
        .iter()
        .map(|line| parse_region(line))
        .collect();

    let presents: Vec<Present> = chunks.map(parse_present).collect();

    (presents, regions)
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 536
    let (_presents, regions) = parse_input(input);

    // dbg!(&presents);
    // dbg!(&regions[..3]);

    regions
        .into_iter()
        // .take(3)
        .map(|region| {
            // dbg!(&region);
            let total_desired: u16 = region.desired_presents.iter().sum();
            // dbg!(&total_desired);
            let d1 = region.dimensions.0 / 3;
            let d2 = region.dimensions.1 / 3;
            let dims = d1 * d2;
            let ratio = f32::from(total_desired) / f32::from(dims);
            // println!(
            //     "dims {}x{} = {} slots",
            //     region.dimensions.0, region.dimensions.1, dims
            // );
            // println!("ratio = {ratio}");

            // !!!!!! maximum compaction is below 1.25
            // THERE ARE NO RATIOS 1<x<1.34
            if ratio > 1.0 && ratio < 1.34 {
                println!("dangerous ratio! {ratio}");
            }

            if ratio <= 1.0 {
                return 1;
            }

            //
            0
        })
        .sum()
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
