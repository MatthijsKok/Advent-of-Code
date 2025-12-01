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
struct Dialer(pub i32);

impl Dialer {
    pub fn new() -> Self {
        Dialer(50)
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

#[tracing::instrument(skip_all)]
pub fn solve_part1(input: &str) -> u16 {
    let mut counter: u16 = 0;
    let mut dialer = Dialer::new();

    for line in input.lines() {
        let amount_passed_zero = dialer.dial(line.into());
        counter += amount_passed_zero;
    }
    counter
}
