use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weekday {
    Mon = 1,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DayError {
    OutOfRange(u8),
    Unknown(String),
}

impl Weekday {
    pub fn number(self) -> u8 {
        todo!()
    }

    pub fn next(self) -> Weekday {
        todo!()
    }

    pub fn is_weekend(self) -> bool {
        todo!()
    }
}

impl TryFrom<u8> for Weekday {
    type Error = DayError;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl FromStr for Weekday {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

pub fn parse_schedule(s: &str) -> Result<Vec<Weekday>, DayError> {
    todo!()
}
