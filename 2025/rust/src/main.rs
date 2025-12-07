#![allow(unused)]

use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

fn main() {
    let tracing_subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(tracing_subscriber).unwrap();
    info!("Hello, Advent of Code!");

    // let input = include_str!("day01/input.txt");
    // day01::solve_part1(input);
    // day01::solve_part2(input);

    // let input = include_str!("day02/input.txt");
    // day02::solve_part1(input);
    // day02::solve_part2(input);

    // let input = include_str!("day03/input.txt");
    // day03::solve_part1(input);
    // day03::solve_part2(input);

    // let input = include_str!("day04/input.txt");
    // day04::solve_part1(input);
    // day04::solve_part2(input);

    // let input = include_str!("day05/input.txt");
    // day05::solve_part1(input);
    // day05::solve_part2(input);

    let input = include_str!("day06/input.txt");
    day06::solve_part1(input);
    day06::solve_part2(input);

    // let input = include_str!("day07/input.txt");
    // day07::solve_part1(input);
    // day07::solve_part2(input);

    // let input = include_str!("day08/input.txt");
    // day08::solve_part1(input);
    // day08::solve_part2(input);

    // let input = include_str!("day09/input.txt");
    // day09::solve_part1(input);
    // day09::solve_part2(input);

    // let input = include_str!("day10/input.txt");
    // day10::solve_part1(input);
    // day10::solve_part2(input);

    // let input = include_str!("day11/input.txt");
    // day11::solve_part1(input);
    // day11::solve_part2(input);

    // let input = include_str!("day12/input.txt");
    // day12::solve_part1(input);
    // day12::solve_part2(input);

    info!("See you tomorrow!");
}
