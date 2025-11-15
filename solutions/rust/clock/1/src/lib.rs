use std::fmt;

#[derive(Debug)]
pub struct Clock{
    hr: i32,
    min: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_mins = hours*60 + minutes;
        total_mins = total_mins % (24*60);
        if total_mins < 0 {
            total_mins += 24*60;
        }
        let real_hours = total_mins / 60;
        let real_minutes = total_mins % 60;
        Self { hr: real_hours, min: real_minutes }
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hr, self.min+minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.hr >= 10 && self.min >= 10 {
            write!(f, "{}:{}", self.hr, self.min)
        } else if self.hr < 10 && self.min >= 10 {
            write!(f, "0{}:{}", self.hr, self.min)
        } else if self.hr >= 10 && self.min < 10 {
            write!(f, "{}:0{}", self.hr, self.min)
        } else {
            write!(f, "0{}:0{}", self.hr, self.min)
        }
    }
}

impl PartialEq for Clock {
    fn eq(&self, clock: &Clock) -> bool {
        self.min == clock.min && self.hr == clock.hr
    }
}