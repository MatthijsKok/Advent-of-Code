use tracing::trace;

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

        let mut step_amount: u16 = match step {
            DialStep::Left(amount) => amount,
            DialStep::Right(amount) => amount,
        };
        // already count the "hundreds"
        counter_zeros += step_amount.div_euclid(DIAL_SIZE.into());
        step_amount %= &DIAL_SIZE.into();

        // match step {
        //     DialStep::Left(_) => self.0 -= &step_amount.into(),
        //     DialStep::Right(_) => self.0 += &step_amount.into(),
        // };
        // if self.0 < 0 || self.0 > 100 {
        //     counter_zeros += 1;
        // }

        let p = match step {
            DialStep::Left(_) => self.0 - i32::from(step_amount),
            DialStep::Right(_) => self.0 + i32::from(step_amount),
        };
        trace!(p);
        trace!("{}", self.0 + i32::from(step_amount));
        if p == 0 {
            counter_zeros += 1;
        }
        if p > 100 {
            counter_zeros += 1;
        }

        self.0 = p.rem_euclid(DIAL_SIZE.into());
        self.0 = self.0.rem_euclid(DIAL_SIZE.into());
        counter_zeros
    }
}

// 6225 < ANSWER < 6269
// 6226 also no, desperation
//


#[tracing::instrument(skip_all)]
pub fn solve_part1(input: &str) -> u32 {
    let mut counter: u32 = 0;
    let mut dialer = Dialer::new();

    for line in input.lines().take(120) {
        let old_dial = dialer.0;
        // trace!(" line: {:5} dial: {:2} counter: {}", line, dialer.0, counter);
        let amount_passed_zero = dialer.dial(line.into());
        trace!(
            "{:2} {:4} {:2} {}  {}",
            old_dial, line, dialer.0, counter, amount_passed_zero
        );
        counter += &amount_passed_zero.into();
        // if dialer.0 == 0 {
        //     counter += 1;
        // }
        // trace!("");
    }
    counter
}
