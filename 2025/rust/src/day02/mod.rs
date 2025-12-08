use rayon::prelude::*;

fn range_from_string(s: &str) -> std::ops::RangeInclusive<usize> {
    s.split_once('-')
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .map(|(a, b)| a..=b)
        .unwrap()
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part2(input: &str) -> usize {
    // Answer = 15704845910
    input
        .lines()
        .next()
        .unwrap()
        .par_split(',')
        .flat_map(range_from_string)
        .filter(|&id| is_silly_id_part2(id))
        .sum()
}

/// An ID is a silly ID if it is made of _only_
/// some sequence of digits repeated _any_ number of times.
fn is_silly_id_part2(id: usize) -> bool {
    let s = id.to_string();
    for i in 1..=s.len() / 2 {
        if !s.len().is_multiple_of(i) {
            // string has to be exactly divisible by `i`
            continue;
        }
        // split string into chunks of size `i`
        let mut chunks = s.as_bytes().chunks_exact(i);
        let first_chunk = chunks.next().unwrap();
        if chunks.all(|c| c == first_chunk) {
            return true;
        }
    }
    false
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 5398419778
    input
        .lines()
        .next()
        .unwrap()
        .par_split(',')
        .filter(|s| s.len() % 4 != 3)
        .flat_map(range_from_string)
        .filter(|&id| is_silly_id_part1(id))
        .sum()
}

/// An ID is a silly ID if it is made of _only_
/// some sequence of digits repeated twice.
fn is_silly_id_part1(id: usize) -> bool {
    let s = id.to_string();
    if !s.len().is_multiple_of(2) {
        return false;
    }
    let (half1, half2) = s.split_at(s.len() / 2);
    half1 == half2
}

#[test]
fn silly_ids_part1() {
    assert!(!is_silly_id_part1(1));
    assert!(!is_silly_id_part1(111));
    assert!(is_silly_id_part1(1212));
    assert!(is_silly_id_part1(11));
    assert!(!is_silly_id_part1(12312));
}

#[test]
fn examples_given_part1() {
    assert_eq!(
        [
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
        ]
        .into_iter()
        .flat_map(range_from_string)
        .filter(|&id| is_silly_id_part1(id))
        .sum::<usize>(),
        1227775554usize
    );
}

#[test]
fn silly_ids_part2() {
    assert!(!is_silly_id_part2(1));
    assert!(is_silly_id_part2(111));
    assert!(is_silly_id_part2(1212));
    assert!(is_silly_id_part2(11));
    assert!(!is_silly_id_part2(12312));
}

#[test]
fn examples_given_part2() {
    assert_eq!(
        range_from_string("11-22")
            .filter(|&id| is_silly_id_part2(id))
            .collect::<Vec<_>>(),
        [11, 22]
    );
    assert_eq!(
        range_from_string("95-115")
            .filter(|&id| is_silly_id_part2(id))
            .collect::<Vec<_>>(),
        [99, 111]
    );
    assert_eq!(
        range_from_string("998-1012")
            .filter(|&id| is_silly_id_part2(id))
            .collect::<Vec<_>>(),
        [999, 1010]
    );
    assert_eq!(
        range_from_string("1188511880-1188511890")
            .filter(|&id| is_silly_id_part2(id))
            .collect::<Vec<_>>(),
        [1188511885]
    );
    assert_eq!(
        range_from_string("222220-222224")
            .filter(|&id| is_silly_id_part2(id))
            .collect::<Vec<_>>(),
        [222222]
    );
    assert_eq!(
        range_from_string("1698522-1698528")
            .filter(|&id| is_silly_id_part2(id))
            .collect::<Vec<_>>(),
        []
    );
    assert_eq!(
        range_from_string("446443-446449")
            .filter(|&id| is_silly_id_part2(id))
            .collect::<Vec<_>>(),
        [446446]
    );
    assert_eq!(
        range_from_string("38593856-38593862")
            .filter(|&id| is_silly_id_part2(id))
            .collect::<Vec<_>>(),
        [38593859]
    );
    assert_eq!(
        range_from_string("565653-565659")
            .filter(|&id| is_silly_id_part2(id))
            .collect::<Vec<_>>(),
        [565656]
    );
    assert_eq!(
        range_from_string("824824821-824824827")
            .filter(|&id| is_silly_id_part2(id))
            .collect::<Vec<_>>(),
        [824824824]
    );
    assert_eq!(
        range_from_string("2121212118-2121212124")
            .filter(|&id| is_silly_id_part2(id))
            .collect::<Vec<_>>(),
        [2121212121]
    );
}

#[test]
fn examples_given_part2_total() {
    assert_eq!(
        [
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ]
        .into_iter()
        .flat_map(range_from_string)
        .filter(|&id| is_silly_id_part2(id))
        .sum::<usize>(),
        4174379265usize
    );
}

#[test]
fn count_odd_numbers() {
    let day02_input = include_str!("input.txt");
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
