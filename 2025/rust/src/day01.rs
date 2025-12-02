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

#[repr(transparent)]
struct DialerOne(pub u16);

impl DialerOne {
    pub fn new() -> Self {
        DialerOne(50)
    }

    pub fn dial(&mut self, step: DialStep) {
        let step_amount: u16 = match step {
            DialStep::Left(amount) => amount,
            DialStep::Right(amount) => amount,
        } % 100;
        self.0 += 100;
        match step {
            DialStep::Left(_) => self.0 -= step_amount,
            DialStep::Right(_) => self.0 += step_amount,
        };
        self.0 %= 100;
    }
}

#[tracing::instrument(skip_all, ret)]
pub fn solve_part1(input: &str) -> u16 {
    // Answer = 1036
    let mut dialer = DialerOne::new();
    input
        .lines()
        .map(|line| {
            dialer.dial(line.into());
            if dialer.0 == 0 { 1 } else { 0 }
        })
        .sum()
}

#[repr(transparent)]
struct DialerTwo(pub i32);

impl DialerTwo {
    pub fn new() -> Self {
        DialerTwo(50)
    }

    pub fn dial(&mut self, step: DialStep) -> u16 {
        let mut counter_zeros: u16 = 0;

        match step {
            DialStep::Left(amount) => {
                let dist: u16 = if self.0 == 0 { 100 } else { self.0 as u16 };
                // let mut dist: u16 = self.0 as u16;
                // if dist == 0 {
                //     dist = 100
                // };

                if amount >= dist {
                    let remaining: u16 = amount - dist;
                    counter_zeros += (remaining / DIAL_SIZE as u16) + 1;
                }
                // let a = (amount as i32 - self.0).max(0);
                // let _ = amount.saturating_sub(self.0 as u16);

                self.0 = (self.0 - i32::from(amount)).rem_euclid(DIAL_SIZE.into());
            }
            DialStep::Right(amount) => {
                counter_zeros += (self.0 as u16 + amount) / u16::from(DIAL_SIZE);
                self.0 = (self.0 + i32::from(amount)) % i32::from(DIAL_SIZE);
            }
        }
        counter_zeros
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
