use std::array;

#[tracing::instrument(skip_all, ret)]
pub(crate) fn solve_part1(input: &str) -> usize {
    // Answer = 5977759036837
    let input_width = input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .count();
    assert_eq!(input_width, 1000);

    let mut data: [Vec<&str>; 1000] = array::repeat(Vec::new());

    for line in input.lines() {
        for (i, item) in line.split_ascii_whitespace().enumerate() {
            if let Some(data_item) = data.get_mut(i) {
                data_item.push(item);
            }
        }
    }

    data.iter_mut()
        .map(|item| -> usize {
            let operator = item.pop().unwrap();
            let numbers = item.iter().map(|n| n.parse::<usize>().unwrap());
            match operator {
                "*" => numbers.product(),
                "+" => numbers.sum(),
                _ => panic!(),
            }
        })
        .sum()
}

#[tracing::instrument(skip_all, ret)]
pub(crate) fn solve_part2(input: &str) -> usize {
    // Answer = 9630000828442
    let input_width = input.lines().next().unwrap().len();
    let input_height = input.lines().count();

    let char_grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (0..input_width)
        .rev()
        .map(|col| {
            (0..input_height)
                .map(|row| char_grid[row][col])
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .split(|line| line.trim().is_empty())
        .map(|chunk| -> usize {
            let operator = chunk
                .iter()
                .flat_map(|line| line.chars())
                .find(|&char| char == '+' || char == '*')
                .unwrap();
            let numbers = chunk.iter().map(|line| {
                line.chars()
                    .filter(|char| char.is_ascii_digit())
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
            });
            match operator {
                '*' => numbers.product(),
                '+' => numbers.sum(),
                _ => panic!(),
            }
        })
        .sum()
}

#[test]
fn part1_examples() {}

#[test]
fn part2_examples() {}
