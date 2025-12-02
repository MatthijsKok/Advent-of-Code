use rayon::prelude::*;
use tracing::trace;

#[tracing::instrument(skip_all)]
pub(crate) fn solve_part1(day02_input: &str) -> usize {
    let ranges = day02_input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .collect::<Vec<_>>();

    let mut counter: usize = 0;

    ranges.into_iter().for_each(|range| {
        for num in range_from_string(range) {
            if is_silly_id(&num.to_string()) {
                counter += num;
            }
        }

        // let mut range_counter: usize = 0;
        // let (num1, num2) = range
        //     .split_once('-')
        //     .map(|(a, b)| {
        //         (
        //             a.parse::<usize>().expect("range start not usize"),
        //             b.parse::<usize>().expect("range end not usize"),
        //         )
        //     })
        //     .unwrap();
        // for num in num1..=num2 {
        //     let num_str = num.to_string();
        //     if is_silly_id(num_str.as_str()) {
        //         range_counter += 1;
        //         trace!("{}   {}", num_str.as_str(), &num_str);
        //     }
        // }
        // // TODO: parallelize with rayon
        // counter += range_counter;
    });
    counter
}

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

/// An ID is a silly ID if it is made of _only_ some sequence of digits repeated twice.
fn is_silly_id(id: &str) -> bool {
    let (half1, half2) = id.split_at(id.len() / 2);
    half1 == half2
}

#[test]
fn silly_ids() {
    assert!(!is_silly_id("1"));
    assert!(!is_silly_id("111"));
    assert!(is_silly_id("1212"));
    assert!(is_silly_id("11"));
    assert!(!is_silly_id("12312"));
}

#[test]
fn examples_given() {
    for num in range_from_string("11-22") {
        if is_silly_id(&num.to_string()) {
            println!("{}", num);
        }
    }

    assert_eq!(
        range_from_string("11-22")
            .chain(range_from_string("95-115"))
            .chain(range_from_string("998-1012"))
            .chain(range_from_string("1188511880-1188511890"))
            .chain(range_from_string("222220-222224"))
            .chain(range_from_string("1698522-1698528"))
            .chain(range_from_string("446443-446449"))
            .chain(range_from_string("38593856-38593862"))
            .filter(|num| is_silly_id(&num.to_string()))
            .sum::<usize>(),
        1227775554usize
    );
}
