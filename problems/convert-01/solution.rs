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
        self as u8
    }

    pub fn next(self) -> Weekday {
        match self {
            Weekday::Mon => Weekday::Tue,
            Weekday::Tue => Weekday::Wed,
            Weekday::Wed => Weekday::Thu,
            Weekday::Thu => Weekday::Fri,
            Weekday::Fri => Weekday::Sat,
            Weekday::Sat => Weekday::Sun,
            Weekday::Sun => Weekday::Mon,
        }
    }
}

impl TryFrom<u8> for Weekday {
    type Error = DayError;

    fn try_from(n: u8) -> Result<Self, Self::Error> {
        match n {
            1 => Ok(Weekday::Mon),
            2 => Ok(Weekday::Tue),
            3 => Ok(Weekday::Wed),
            4 => Ok(Weekday::Thu),
            5 => Ok(Weekday::Fri),
            6 => Ok(Weekday::Sat),
            7 => Ok(Weekday::Sun),
            _ => Err(DayError::OutOfRange(n)),
        }
    }
}

impl FromStr for Weekday {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "mon" => Ok(Weekday::Mon),
            "tue" => Ok(Weekday::Tue),
            "wed" => Ok(Weekday::Wed),
            "thu" => Ok(Weekday::Thu),
            "fri" => Ok(Weekday::Fri),
            "sat" => Ok(Weekday::Sat),
            "sun" => Ok(Weekday::Sun),
            _ => Err(DayError::Unknown(s.to_string())),
        }
    }
}
