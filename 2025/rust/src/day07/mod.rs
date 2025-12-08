use std::collections::HashSet;

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

#[tracing::instrument(skip_all, ret)]
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

    let start_col = manifold_grid[0]
        .iter()
        .position(|cell| cell == &GridCell::Start)
        .unwrap();
    let mut active_beams: HashSet<usize> = HashSet::with_capacity(128);
    active_beams.insert(start_col);

    for row in manifold_grid {
        let mut new_beams: HashSet<usize> = HashSet::new();
        for &col in &active_beams {
            if row[col] == GridCell::Splitter {
                split_count += 1;
                new_beams.insert(col - 1);
                new_beams.insert(col + 1);
            } else {
                new_beams.insert(col);
            }
        }
        active_beams = new_beams;
    }

    split_count
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = ?
    let mut split_count: usize = 0;
    let manifold_grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| GridCell::try_from(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let start_col = manifold_grid[0]
        .iter()
        .position(|cell| cell == &GridCell::Start)
        .unwrap();
    let mut active_beams: Vec<usize> = Vec::with_capacity(256);
    active_beams.push(start_col);

    for row in &manifold_grid {
        let mut new_beams: Vec<usize> = Vec::with_capacity(256);
        for &col in &active_beams {
            if row[col] == GridCell::Splitter {
                split_count += 1;
                new_beams.push(col - 1);
                new_beams.push(col + 1);
            } else {
                new_beams.push(col);
            }
        }
        active_beams = new_beams;
    }

    split_count
}

#[test]
fn part1_examples() {}

#[test]
fn part2_examples() {}
