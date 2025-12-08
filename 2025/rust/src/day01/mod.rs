#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_wrap)]

const DIAL_SIZE: u8 = 100;

enum DialStep {
    Left(u16),
    Right(u16),
}

impl TryFrom<&str> for DialStep {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (direction, amount) = value.split_at(1);
        let clicks: u16 = amount.parse().expect("dial step amount not a u16");
        match direction {
            "L" => Ok(Self::Left(clicks)),
            "R" => Ok(Self::Right(clicks)),
            _ => Err(()),
        }
    }
}

struct DialerOne(pub i16);

impl DialerOne {
    pub const fn new() -> Self {
        Self(50)
    }

    #[inline]
    pub fn dial_bytes(&mut self, line: &[u8]) -> u16 {
        assert!(2 <= line.len() && line.len() <= 4);
        let mut amount: i16 = 0;
        let is_right = line[0] == b'R';
        for &b in &line[1..] {
            amount = amount * 10 + i16::from(b - b'0');
        }
        let step = amount * (2 * i16::from(is_right) - 1);
        self.0 = (self.0 + step).rem_euclid(100);
        u16::from(self.0 == 0)
    }
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> u16 {
    // Answer = 1036
    let mut dialer = DialerOne::new();
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| dialer.dial_bytes(line))
        .sum()
}

struct DialerTwo(pub i16);

impl DialerTwo {
    pub const fn new() -> Self {
        Self(50)
    }

    pub fn dial(&mut self, step: &DialStep) -> u16 {
        match step {
            DialStep::Left(amount) => {
                let dist: u16 = if self.0 == 0 { 100 } else { self.0 as u16 };
                let zeros = (amount + 100 - dist) / 100;
                self.0 = (self.0 - *amount as i16).rem_euclid(DIAL_SIZE.into());
                zeros
            }
            DialStep::Right(amount) => {
                let zeros = (self.0 as u16 + amount) / u16::from(DIAL_SIZE);
                self.0 = (self.0 + *amount as i16) % i16::from(DIAL_SIZE);
                zeros
            }
        }
    }
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part2(input: &str) -> u16 {
    // Answer = 6228
    let mut dialer = DialerTwo::new();
    input
        .lines()
        .map(|line| dialer.dial(&line.try_into().unwrap()))
        .sum()
}
