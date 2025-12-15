#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum GridCell {
    Start,
    Splitter,
    Empty,
}

impl TryFrom<char> for GridCell {
    type Error = char;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Start),
            '^' => Ok(Self::Splitter),
            '.' => Ok(Self::Empty),
            _ => Err(value),
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 1675
    let mut split_count: usize = 0;
    let manifold_grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| GridCell::try_from(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = manifold_grid[0].len();
    let start_col = manifold_grid[0]
        .iter()
        .position(|cell| cell == &GridCell::Start)
        .unwrap();

    let mut active_beams: Vec<bool> = vec![false; width];
    active_beams[start_col] = true;

    for row in manifold_grid {
        let mut new_beams: Vec<bool> = vec![false; width];
        for col in 0..width {
            if !active_beams[col] {
                continue;
            }
            if row[col] == GridCell::Splitter {
                split_count += 1;
                new_beams[col - 1] = true;
                new_beams[col + 1] = true;
            } else {
                new_beams[col] = true;
            }
        }
        active_beams = new_beams;
    }

    split_count
}

#[tracing::instrument(skip_all)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = 187987920774390
    let manifold_grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| GridCell::try_from(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = manifold_grid[0].len();
    let start_col = manifold_grid[0]
        .iter()
        .position(|cell| cell == &GridCell::Start)
        .unwrap();

    let mut timelines: Vec<usize> = vec![0; width];
    timelines[start_col] = 1;

    for row in &manifold_grid {
        let mut new_timelines: Vec<usize> = vec![0; width];
        for (col, &count) in timelines.iter().enumerate() {
            if count == 0 {
                continue;
            }
            if row[col] == GridCell::Splitter {
                new_timelines[col - 1] += count;
                new_timelines[col + 1] += count;
            } else {
                new_timelines[col] += count;
            }
        }
        timelines = new_timelines;
    }

    timelines.iter().sum()
}

#[test]
fn part1_examples() {}

#[test]
fn part2_examples() {}
