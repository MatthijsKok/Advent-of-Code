#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

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

    // let span = tracing::info_span!("total_runtime");
    // let _enter = span.enter();

    let input = include_str!("day01/input.txt");
    assert_eq!(day01::solve_part1(input), 1036);
    assert_eq!(day01::solve_part2(input), 6228);

    let input = include_str!("day02/input.txt");
    assert_eq!(day02::solve_part1(input), 5_398_419_778);
    assert_eq!(day02::solve_part2(input), 15_704_845_910);

    let input = include_str!("day03/input.txt");
    assert_eq!(day03::solve_part1(input), 17330);
    assert_eq!(day03::solve_part2(input), 171_518_260_283_767);

    let input = include_str!("day04/input.txt");
    assert_eq!(day04::solve_part1(input), 1474);
    assert_eq!(day04::solve_part2(input), 8910);

    let input = include_str!("day05/input.txt");
    assert_eq!(day05::solve_part1(input), 739);
    assert_eq!(day05::solve_part2(input), 344_486_348_901_788);

    let input = include_str!("day06/input.txt");
    assert_eq!(day06::solve_part1(input), 5_977_759_036_837);
    assert_eq!(day06::solve_part2(input), 9_630_000_828_442);

    let input = include_str!("day07/input.txt");
    assert_eq!(day07::solve_part1(input), 1675);
    assert_eq!(day07::solve_part2(input), 187_987_920_774_390);

    let input = include_str!("day08/input.txt");
    assert_eq!(day08::solve_part1(input), 171_503);
    assert_eq!(day08::solve_part2(input), 9_069_509_600);

    let input = include_str!("day09/input.txt");
    assert_eq!(day09::solve_part1(input), 4_725_826_296);
    assert_eq!(day09::solve_part2(input), 1_637_556_834);

    let input = include_str!("day10/input.txt");
    assert_eq!(day10::solve_part1(input), 404);
    assert_eq!(day10::solve_part2(input), 16474);

    let input = include_str!("day11/input.txt");
    assert_eq!(day11::solve_part1(input), 607);
    assert_eq!(day11::solve_part2(input), 506_264_456_238_938);

    let input = include_str!("day12/input.txt");
    assert_eq!(day12::solve_part1(input), 536);
    assert_eq!(day12::solve_part2(input), 0);

    info!("See you next year!");
}
