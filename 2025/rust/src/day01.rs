const DIAL_SIZE: u8 = 100;

enum DialStep {
    Left(u16),
    Right(u16),
}

impl From<&str> for DialStep {
    fn from(value: &str) -> Self {
        let (direction, amount) = value.split_at(1);
        let clicks: u16 = amount.parse().expect("dial step amount not a u16");
        match direction {
            "L" => Self::Left(clicks),
            "R" => Self::Right(clicks),
            _ => panic!("dial direction is not L or R"),
        }
    }
}

struct DialerOne(pub i16);

impl DialerOne {
    pub fn new() -> Self {
        DialerOne(50)
    }

    pub fn dial(&mut self, step: DialStep) {
        let step_amount: i16 = match step {
            DialStep::Left(amount) => amount as i16,
            DialStep::Right(amount) => amount as i16,
        } % 100;
        self.0 += 100;
        match step {
            DialStep::Left(_) => self.0 -= step_amount,
            DialStep::Right(_) => self.0 += step_amount,
        };
        self.0 %= 100;
    }

    pub fn dial_bytes(&mut self, line: &[u8]) {
        assert!(2 <= line.len() && line.len() <= 4);
        let mut amount: i16 = 0;
        let is_right = line[0] == b'R';
        for &b in &line[1..] {
            amount = amount * 10 + (b - b'0') as i16;
        }
        let step = amount * (2 * is_right as i16 - 1);
        self.0 = (self.0 + step).rem_euclid(100);
    }
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> u16 {
    // Answer = 1036
    let mut dialer = DialerOne::new();
    // let _: u16 = input
    //     .lines()
    //     .map(|line| {
    //         dialer.dial(line.into());
    //         u16::from(dialer.0 == 0)
    //     })
    //     .sum();
    input
        .as_bytes()
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            dialer.dial_bytes(line);
            u16::from(dialer.0 == 0)
        })
        .sum()
}

struct DialerTwo(pub i16);

impl DialerTwo {
    pub fn new() -> Self {
        DialerTwo(50)
    }

    pub fn dial(&mut self, step: DialStep) -> u16 {
        match step {
            DialStep::Left(amount) => {
                let dist: u16 = if self.0 == 0 { 100 } else { self.0 as u16 };
                let zeros = (amount + 100 - dist) / 100;
                self.0 = (self.0 - amount as i16).rem_euclid(DIAL_SIZE.into());
                zeros
            }
            DialStep::Right(amount) => {
                let zeros = (self.0 as u16 + amount) / u16::from(DIAL_SIZE);
                self.0 = (self.0 + amount as i16) % i16::from(DIAL_SIZE);
                zeros
            }
        }
    }
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part2(input: &str) -> u16 {
    // Answer = 6228
    let mut counter: u16 = 0;
    let mut dialer = DialerTwo::new();

    for line in input.lines() {
        let amount_passed_zero = dialer.dial(line.into());
        counter += amount_passed_zero;
    }
    counter
}
