const DIAL_SIZE: i16 = 100;

enum DialStep {
    Left(i16),
    Right(i16),
}

impl From<&str> for DialStep {
    fn from(value: &str) -> Self {
        let (direction, amount) = value.split_at(1);
        let clicks: i16 = amount.parse().expect("dial step amount not a i16");
        match direction {
            "L" => Self::Left(clicks),
            "R" => Self::Right(clicks),
            _ => panic!("dial direction is not L or R"),
        }
    }
}

#[repr(transparent)]
struct Dialer(pub i16);

impl Dialer {
    pub fn new() -> Self {
        Dialer(50)
    }

    pub fn dial(&mut self, step: DialStep) {
        let step_amount = match step {
            DialStep::Left(amount) => amount,
            DialStep::Right(amount) => amount,
        } % DIAL_SIZE;
        self.0 += DIAL_SIZE;
        match step {
            DialStep::Left(_) => self.0 -= step_amount,
            DialStep::Right(_) => self.0 += step_amount,
        };
        self.0 %= DIAL_SIZE;
    }
}

#[tracing::instrument(skip_all)]
pub fn solve_part1(input: &str) -> i32 {
    let mut counter = 0;
    let mut dialer = Dialer::new();

    for line in input.lines() {
        dialer.dial(line.into());
        if dialer.0 == 0 {
            counter += 1;
        }
    }
    counter
}
