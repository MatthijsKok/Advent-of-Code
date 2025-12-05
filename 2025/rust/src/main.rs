#![allow(unused)]

use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let tracing_subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(tracing_subscriber).unwrap();
    info!("Hello, Advent of Code!");

    let day01_input = include_str!("day01/input.txt");
    day01::solve_part1(day01_input);
    day01::solve_part2(day01_input);

    let day02_input = include_str!("day02/input.txt");
    day02::solve_part1(day02_input);
    day02::solve_part2(day02_input);

    let day03_input = include_str!("day03/input.txt");
    day03::solve_part1(day03_input);
    day03::solve_part2(day03_input);

    let day04_input = include_str!("day04/input.txt");
    day04::solve_part1(day04_input);
    day04::solve_part2(day04_input);

    info!("See you tomorrow!");
}
