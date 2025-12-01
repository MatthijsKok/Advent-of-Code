use tracing::{info, trace};
use tracing_subscriber::fmt::format::FmtSpan;

mod day01;

fn main() {
    let tracing_subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(tracing_subscriber)
        .expect("setting default subscriber failed");
    trace!("Tracing initialized");

    info!("Hello, Advent of Code!");
    let day01_part1_input = include_str!("day01_input.txt");
    let day01_part1 = day01::solve_part1(day01_part1_input);
    info!("Day 01 Part 1: {day01_part1}");

    trace!("See you tomorrow!");
}
