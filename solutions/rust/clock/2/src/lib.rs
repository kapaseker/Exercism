use std::fmt;

#[derive(Debug)]
pub struct Clock(i32, i32);

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.0, self.1)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut add_hour = minutes / 60;
        if minutes < 0 && (minutes % 60 != 0) {
            add_hour -= 1;
        }
        let m = (minutes % 60 + 60) % 60;
        let h = ((add_hour + hours) % 24 + 24) % 24;
        return Self(h, m);
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Self::new(self.0, self.1 + minutes);
    }
}
