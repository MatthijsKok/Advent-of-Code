use rayon::prelude::*;

fn bank_find_max(bank: &[u8], count: usize) -> usize {
    let n = bank.len();
    let mut result: usize = 0;
    let mut prev_pos = 0;

    for i in 0..count {
        let start = if i == 0 { 0 } else { prev_pos + 1 };
        let end = n - count + i;

        let mut max_digit = 0;
        let mut max_pos = start;
        for (j, &digit) in bank[start..=end].iter().enumerate() {
            if digit > max_digit {
                max_digit = digit;
                max_pos = start + j;
            }
        }

        result = result * 10 + (max_digit - b'0') as usize;
        prev_pos = max_pos;
    }

    result
}

fn solve(input: &str, battery_size: usize) -> usize {
    input
        .par_lines()
        .map(|bank| bank.as_bytes())
        .map(|digits| bank_find_max(digits, battery_size))
        .sum()
}

#[tracing::instrument(skip_all, ret)]
pub(crate) fn solve_part1(input: &str) -> usize {
    // Answer = 17330
    solve(input, 2)
}

#[tracing::instrument(skip_all, ret)]
pub(crate) fn solve_part2(input: &str) -> usize {
    // Answer = 171518260283767
    solve(input, 12)
}

#[test]
fn part1_examples() {
    assert_eq!(bank_find_max(b"987654321111111", 2), 98);
    assert_eq!(bank_find_max(b"811111111111119", 2), 89);
    assert_eq!(bank_find_max(b"234234234234278", 2), 78);
    assert_eq!(bank_find_max(b"818181911112111", 2), 92);
}

#[test]
fn part2_examples() {
    assert_eq!(bank_find_max(b"987654321111111", 12), 987654321111);
    assert_eq!(bank_find_max(b"811111111111119", 12), 811111111119);
    assert_eq!(bank_find_max(b"234234234234278", 12), 434234234278);
    assert_eq!(bank_find_max(b"818181911112111", 12), 888911112111);
}
