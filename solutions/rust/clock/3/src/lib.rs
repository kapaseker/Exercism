use std::fmt;

const MINUTE_IN_DAY: i32 = 1440;
const MINUTE_IN_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock(i32);

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", (self.0 / 60) % 24, self.0 % 60)
    }
}

impl Clock {

    pub fn new(hours: i32, minutes: i32) -> Self {
        return Self::make_clock(hours * MINUTE_IN_HOUR + minutes);
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Self::make_clock(self.0 + minutes);
    }

    fn make_clock(minutes: i32) -> Self {
        return Self((minutes % MINUTE_IN_DAY + MINUTE_IN_DAY) % MINUTE_IN_DAY);
    }
}