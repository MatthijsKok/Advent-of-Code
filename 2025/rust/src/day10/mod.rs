use std::collections::VecDeque;
use std::fmt::Debug;

use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum IndicatorState {
    Off,
    On,
}

impl std::ops::Not for IndicatorState {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        }
    }
}

impl TryFrom<char> for IndicatorState {
    type Error = char;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::On),
            '.' => Ok(Self::Off),
            _ => Err(value),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
struct Machine {
    indicator_size: usize,
    indicator_target_state: usize,
    button_sets: Vec<Vec<u8>>,
    joltages: Vec<usize>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let mut chunks = line.split_ascii_whitespace();
        let indicators = chunks
            .next()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .chars()
            .map(|c| c.try_into().unwrap())
            .collect::<Vec<IndicatorState>>();
        let joltages = chunks
            .next_back()
            .unwrap()
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let mut button_sets: Vec<Vec<u8>> = chunks
            .map(|chunk| {
                chunk
                    .chars()
                    .filter_map(|c| c.to_digit(10).and_then(|d| u8::try_from(d).ok()))
                    .collect()
            })
            .collect();
        button_sets.sort_by_key(Vec::len);
        button_sets.reverse();

        let indicator_size = indicators.len();
        let indicator_target_state = indicators.iter().enumerate().fold(0usize, |acc, (i, s)| {
            if *s == IndicatorState::On {
                acc | (1usize << i)
            } else {
                acc
            }
        });

        Self {
            indicator_size,
            indicator_target_state,
            button_sets,
            joltages,
        }
    }

    fn fewest_presses(&self) -> usize {
        let mut button_masks: Vec<usize> = Vec::with_capacity(self.button_sets.len());
        for button in &self.button_sets {
            let mut mask = 0usize;
            for &i in button {
                if i as usize >= self.indicator_size {
                    continue;
                }
                mask |= 1usize << i;
            }
            button_masks.push(mask);
        }

        let max_states = 1usize << self.indicator_size;
        let mut dist = vec![usize::MAX; max_states];
        let mut q = VecDeque::with_capacity(64);

        dist[0] = 0;
        q.push_back(0usize);

        while let Some(state) = q.pop_front() {
            if state == self.indicator_target_state {
                return dist[state];
            }
            let d = dist[state];
            for &bm in &button_masks {
                let ns = state ^ bm;
                if dist[ns] == usize::MAX {
                    dist[ns] = d + 1;
                    q.push_back(ns);
                }
            }
        }

        unreachable!("target indicator state unreachable");
    }

    #[tracing::instrument(skip_all)]
    fn fewest_presses_joltage(&self) -> usize {
        let targets = &self.joltages;
        let counters = targets.len();

        let buttons = &self.button_sets;

        let mut rem = targets.clone();

        let mut best = usize::MAX;

        let mut rem_g = rem.clone();
        let mut count = 0usize;
        for b in buttons {
            let max_times = b.iter().map(|&i| rem_g[i as usize]).min().unwrap_or(0);
            if max_times > 0 {
                for &i in b {
                    rem_g[i as usize] = rem_g[i as usize].saturating_sub(max_times);
                }
                count += max_times;
            }
        }
        if rem_g.iter().all(|&x| x == 0) {
            best = count;
        }

        if best == usize::MAX {
            let mut rem2 = rem.clone();
            let mut count2 = 0usize;
            let mut counter_buttons: Vec<Vec<usize>> = vec![vec![]; counters];
            for (j, b) in buttons.iter().enumerate() {
                for &i in b {
                    counter_buttons[i as usize].push(j);
                }
            }
            let mut safe = true;
            while rem2.iter().any(|&x| x > 0) {
                let (imax, &maxr) = rem2.iter().enumerate().max_by_key(|&(_, &v)| v).unwrap();
                if maxr == 0 {
                    break;
                }
                if let Some(&bj) = counter_buttons[imax].first() {
                    let times = buttons[bj]
                        .iter()
                        .map(|&i| rem2[i as usize])
                        .min()
                        .unwrap_or(0);
                    if times == 0 {
                        safe = false;
                        break;
                    }
                    for &i in &buttons[bj] {
                        rem2[i as usize] = rem2[i as usize].saturating_sub(times);
                    }
                    count2 += times;
                } else {
                    safe = false;
                    break;
                }
            }
            if safe && rem2.iter().all(|&x| x == 0) {
                best = count2;
            }
        }

        if best == usize::MAX {
            best = rem.iter().sum();
        }

        dfs(buttons, &mut rem, 0, 0, &mut best);

        best
    }
}

impl Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Machine")
            .field(
                "indicator_size",
                &format_args!("{:?}", &self.indicator_size),
            )
            .field(
                "indicator_target_state",
                &format_args!("{:?}", &self.indicator_target_state),
            )
            .field("button_sets", &format_args!("{:?}", &self.button_sets))
            .field("joltages", &format_args!("{:?}", &self.joltages))
            .finish()
    }
}

fn compute_lb(buttons: &[Vec<u8>], rem_vec: &[usize], start_btn: usize) -> usize {
    let max_rem = *rem_vec.iter().max().unwrap_or(&0usize);
    let sum_rem: usize = rem_vec.iter().sum();
    let mut max_set_size = 1usize;
    for b in buttons.iter().skip(start_btn) {
        max_set_size = max_set_size.max(b.len());
    }
    sum_rem.div_ceil(max_set_size).max(max_rem)
}

fn dfs(buttons: &Vec<Vec<u8>>, rem: &mut Vec<usize>, idx: usize, current: usize, best: &mut usize) {
    if current >= *best {
        return;
    }
    if rem.iter().all(|&x| x == 0) {
        *best = current;
        return;
    }
    if idx >= buttons.len() {
        return;
    }

    let lb = compute_lb(buttons, rem, idx);
    if current + lb >= *best {
        return;
    }

    let max_times = buttons[idx]
        .iter()
        .map(|&i| rem[i as usize])
        .min()
        .unwrap_or(0);

    for times in (0..=max_times).rev() {
        if current + times >= *best {
            continue;
        }
        if times > 0 {
            for &i in &buttons[idx] {
                rem[i as usize] -= times;
            }
        }
        let new_lb = compute_lb(buttons, rem, idx + 1);
        if current + times + new_lb < *best {
            dfs(buttons, rem, idx + 1, current + times, best);
        }
        if times > 0 {
            for &i in &buttons[idx] {
                rem[i as usize] += times;
            }
        }
    }
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> usize {
    // Answer = 404
    // NOTES:
    // There can be at most 9 indicators
    // -> indicator index is a single digit number
    let machines = input.lines().map(Machine::parse).collect::<Vec<_>>();
    // dbg!(&machines[0..2]);
    machines.iter().map(Machine::fewest_presses).sum()
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part2(input: &str) -> usize {
    let machines = input.lines().map(Machine::parse).collect::<Vec<_>>();
    machines
        .par_iter()
        .map(Machine::fewest_presses_joltage)
        .sum()
}

#[cfg(test)]
const EXAMPLE: &str = "\
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

#[test]
fn part1_examples() {
    let line1 = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
    assert_eq!(solve_part1(line1), 2);
    let line2 = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
    assert_eq!(solve_part1(line2), 3);
    let line3 = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    assert_eq!(solve_part1(line3), 2);
    assert_eq!(solve_part1(EXAMPLE), 7);
}

#[test]
fn part2_examples() {
    let line1 = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
    assert_eq!(solve_part2(line1), 10);
    let line2 = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
    assert_eq!(solve_part2(line2), 12);
    let line3 = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    assert_eq!(solve_part2(line3), 11);
    assert_eq!(solve_part2(EXAMPLE), 33);
}
