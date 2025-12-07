use std::{array, panic};

use rayon::str::ParallelString;

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
            let sign = item.pop().unwrap();
            let numbers: Vec<usize> = item
                .iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            match sign {
                "*" => numbers.iter().product(),
                "+" => numbers.iter().sum(),
                _ => panic!(),
            }
        })
        .sum()
}

#[tracing::instrument(skip_all, ret)]
pub(crate) fn solve_part2(input: &str) -> usize {
    // Answer = 9630000828442
    let input_width = input.lines().next().unwrap().len();
    assert_eq!(input_width, 3746);
    assert!(input.lines().all(|line| line.len() == 3746));
    let input_height = input.lines().count();
    let char_grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // let rotated_grid: Vec<Vec<char>> = (0..input_width)
    //     .rev()
    //     .map(|col| (0..input_height).map(|row| char_grid[row][col]).collect())
    //     .collect();

    // let rotated_lines = rotated_grid
    //     .into_iter()
    //     .map(|line| line.iter().collect::<String>())
    //     .collect::<Vec<_>>();
    // // dbg!(&rotated_lines);
    // rotated_lines
    //     .split(|line| line.trim().is_empty())
    //     .filter(|&chunk| !chunk.is_empty())
    //     .map(|chunk| -> usize {
    //         //

    //         0
    //     })
    //     .sum()

    (0..input_width)
        .rev()
        .map(|col| {
            (0..input_height)
                .map(|row| char_grid[row][col])
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .split(|line| line.trim().is_empty())
        .filter(|&chunk| !chunk.is_empty())
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
