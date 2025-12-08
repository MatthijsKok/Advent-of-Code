use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum GridCell {
    Start,
    Splitter,
    Empty,
}

impl From<&str> for GridCell {
    fn from(value: &str) -> Self {
        match value {
            "S" => Self::Start,
            "^" => Self::Splitter,
            "." => Self::Empty,
            _ => panic!(),
        }
    }
}

impl From<char> for GridCell {
    fn from(value: char) -> Self {
        match value {
            'S' => Self::Start,
            '^' => Self::Splitter,
            '.' => Self::Empty,
            _ => panic!(),
        }
    }
}

#[tracing::instrument(skip_all, ret)]
pub(crate) fn solve_part1(input: &str) -> usize {
    // Answer = 1675
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    assert_eq!(width, 141);
    assert_eq!(height, 142);

    let manifold_grid = input
        .lines()
        .map(|line| line.chars().map(GridCell::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_col = manifold_grid
        .iter()
        .find(|line| line.contains(&GridCell::Start))
        .unwrap()
        .iter()
        .position(|cell| cell == &GridCell::Start)
        .unwrap();

    let mut split_count: usize = 0;
    let mut active_beams: HashSet<usize> = HashSet::with_capacity(128);
    active_beams.insert(start_col);

    for (row, _) in input.lines().enumerate() {
        let mut new_beams: HashSet<usize> = HashSet::new();
        for &col in &active_beams {
            if manifold_grid[row][col] == GridCell::Splitter {
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
pub(crate) fn solve_part2(input: &str) -> usize {
    // Answer = ?
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    assert_eq!(width, 141);
    assert_eq!(height, 142);
    0
}

#[test]
fn part1_examples() {}

#[test]
fn part2_examples() {}
