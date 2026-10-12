use crate::judge::{case, check_eq};
use crate::user::{DayError, Weekday};

const ALL: [Weekday; 7] = [
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
    Weekday::Sat,
    Weekday::Sun,
];

fn unknown(s: &str) -> DayError {
    DayError::Unknown(s.to_string())
}

fn parse(s: &str) -> Result<Weekday, DayError> {
    s.parse()
}

pub fn run() {
    case("sample-1", || {
        check_eq(Weekday::Thu.number(), 4)?;
        check_eq(Weekday::Fri.next(), Weekday::Sat)?;
        check_eq(Weekday::try_from(7), Ok(Weekday::Sun))?;
        let r: Result<Weekday, _> = 0u8.try_into();
        check_eq(r, Err(DayError::OutOfRange(0)))
    });
    case("sample-2", || {
        check_eq("fri".parse::<Weekday>(), Ok(Weekday::Fri))?;
        check_eq("Funday".parse::<Weekday>(), Err(unknown("Funday")))
    });
    case("hidden-1", || {
        // Every number in 1..=7 maps to its day and back.
        for (i, day) in ALL.into_iter().enumerate() {
            let n = i as u8 + 1;
            check_eq(day.number(), n)?;
            check_eq(Weekday::try_from(n), Ok(day))?;
            let via_into: Result<Weekday, DayError> = n.try_into();
            check_eq(via_into, Ok(day))?;
        }
        Ok(())
    });
    case("hidden-2", || {
        check_eq(Weekday::try_from(0), Err(DayError::OutOfRange(0)))?;
        check_eq(Weekday::try_from(8), Err(DayError::OutOfRange(8)))?;
        check_eq(Weekday::try_from(255), Err(DayError::OutOfRange(255)))?;
        let r: Result<Weekday, DayError> = 100u8.try_into();
        check_eq(r, Err(DayError::OutOfRange(100)))
    });
    case("hidden-3", || {
        // next() goes around the week, and Sun wraps to Mon.
        let got: Vec<Weekday> = ALL.into_iter().map(Weekday::next).collect();
        let expected = [
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
            Weekday::Mon,
        ];
        check_eq(got, expected)
    });
    case("hidden-4", || {
        // ASCII case-insensitive.
        check_eq(parse("MON"), Ok(Weekday::Mon))?;
        check_eq(parse("mOn"), Ok(Weekday::Mon))?;
        check_eq(parse("Tue"), Ok(Weekday::Tue))?;
        check_eq(parse("WED"), Ok(Weekday::Wed))?;
        check_eq(parse("thu"), Ok(Weekday::Thu))?;
        check_eq(parse("fRI"), Ok(Weekday::Fri))?;
        check_eq(parse("Sat"), Ok(Weekday::Sat))?;
        check_eq(parse("sUn"), Ok(Weekday::Sun))
    });
    case("hidden-5", || {
        // No trimming, no full names, and the error keeps the input unchanged.
        check_eq(parse(" mon"), Err(unknown(" mon")))?;
        check_eq(parse("sun\n"), Err(unknown("sun\n")))?;
        check_eq(parse("MONDAY"), Err(unknown("MONDAY")))?;
        check_eq(parse("Tuesday"), Err(unknown("Tuesday")))?;
        check_eq(parse("Mo"), Err(unknown("Mo")))?;
        check_eq(parse(""), Err(unknown("")))?;
        check_eq(parse("é"), Err(unknown("é")))?;
        check_eq(parse("Mön"), Err(unknown("Mön")))
    });
}
