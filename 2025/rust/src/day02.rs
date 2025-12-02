use rayon::prelude::*;

fn range_from_string(s: &str) -> std::ops::RangeInclusive<usize> {
    let (num1, num2) = s
        .split_once('-')
        .map(|(a, b)| {
            (
                a.parse::<usize>().expect("range start not usize"),
                b.parse::<usize>().expect("range end not usize"),
            )
        })
        .unwrap();
    num1..=num2
}

#[tracing::instrument(skip_all)]
pub(crate) fn solve_part1(input: &str) -> usize {
    // Answer = 5398419778
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .filter(|s| s.len() % 4 != 3)
        .par_bridge()
        .flat_map(range_from_string)
        .filter(is_silly_id_part1)
        .sum()
}

/// An ID is a silly ID if it is made of _only_
/// some sequence of digits repeated twice.
fn is_silly_id_part1(id: &usize) -> bool {
    let s = id.to_string();
    let (half1, half2) = s.split_at(s.len() / 2);
    half1 == half2
}

#[test]
fn silly_ids_part1() {
    assert!(!is_silly_id_part1(&1));
    assert!(!is_silly_id_part1(&111));
    assert!(is_silly_id_part1(&1212));
    assert!(is_silly_id_part1(&11));
    assert!(!is_silly_id_part1(&12312));
}

#[test]
fn examples_given_part1() {
    assert_eq!(
        range_from_string("11-22")
            .chain(range_from_string("95-115"))
            .chain(range_from_string("998-1012"))
            .chain(range_from_string("1188511880-1188511890"))
            .chain(range_from_string("222220-222224"))
            .chain(range_from_string("1698522-1698528"))
            .chain(range_from_string("446443-446449"))
            .chain(range_from_string("38593856-38593862"))
            .filter(is_silly_id_part1)
            .sum::<usize>(),
        1227775554usize
    );
}

#[test]
fn count_odd_numbers() {
    let day02_input = include_str!("day02_input.txt");
    let mut evens: usize = 0;
    let mut odds: usize = 0;
    for pair in day02_input.lines().next().unwrap().split(',') {
        let (s1, s2) = pair.split_once('-').unwrap();
        if s1.len() % 2 == 1 && s2.len() % 2 == 1 {
            // both range start and range end are numbers with uneven amount of digits.
            // Thus the whole range can be discarded immediately.
            odds += s2.parse::<usize>().unwrap() - s1.parse::<usize>().unwrap();
        } else {
            evens += s2.parse::<usize>().unwrap() - s1.parse::<usize>().unwrap();
        }
    }
    println!("total even: {}", evens); // 1135383
    println!("total odd:  {}", odds); //  923901
    println!(
        "odd percentage: {:.2}%",
        odds as f64 / (evens + odds) as f64 * 100.
    );
    // This early discarding saves:
    // 923901 / (1135383 + 923901) * 100 = 44.87% of checks!
}
