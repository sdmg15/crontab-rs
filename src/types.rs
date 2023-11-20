use core::fmt;
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::RangeInclusive;
pub trait Segment: BuildableSegment + Display {
    fn validate_range(&self, item: &str) -> bool;
    fn max(&self) -> u8;
}

pub trait BuildableSegment {
    fn new(
        range_start: Option<String>,
        range_end: Option<String>,
        step: Option<String>,
        val: Option<String>,
    ) -> Self;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Common<T> {
    pub range_start: Option<String>,
    pub range_end: Option<String>,
    pub step: Option<String>,
    pub val: Option<String>,
    seg_type: std::marker::PhantomData<T>,
}


impl<T> BuildableSegment for Common<T> {
    fn new(
        range_start: Option<String>,
        range_end: Option<String>,
        step: Option<String>,
        val: Option<String>,
    ) -> Self {
        Self {
            range_start,
            range_end,
            step,
            val,
            seg_type: PhantomData,
        }
    }
}
impl<T> Default for Common<T> {
    fn default() -> Self {
        Self {
            range_start: None,
            range_end: None,
            step: None,
            val: None,
            seg_type: PhantomData,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct MinuteType;
pub type Minutes = Common<MinuteType>;
#[derive(Debug, PartialEq, Eq)]
pub struct HourType;
pub type Hour = Common<HourType>;
#[derive(Debug, PartialEq, Eq)]
pub struct DayOfMonthType;
pub type DayOfMonth = Common<DayOfMonthType>;

#[derive(Debug, PartialEq, Eq)]
pub struct DayOfWeekType;
pub type DayOfWeek = Common<DayOfWeekType>;
#[derive(Debug, PartialEq, Eq)]
pub struct MonthType;
pub type Month = Common<MonthType>;

impl Segment for Minutes {
    fn validate_range(&self, elem: &str) -> bool {
        let valid_range = 0u8..=59u8;
        match elem {
            "*" => true,
            _ => valid_range.contains(&elem.parse::<u8>().unwrap()),
        }
    }

    fn max(&self) -> u8 {
        59
    }
}

impl Display for Minutes { 

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(r) = &self.val {
            if r == "*" {
                return write!(f, "At every minute")
            }
            return write!(f, "At minute {r}")
        }

        let mut s = String::new();
        if let Some(r) = &self.range_start {
            s = format!("from {r} through {}", self.range_end.as_ref().unwrap());
        }
        if let Some(r) = &self.step {
            s = format!("At every {r} minute {}", s);
        } else {
            s = format!("At every minute {}", s);
        }
        write!(f, "{s}")
    }
}

impl Segment for Hour {
    fn validate_range(&self, elem: &str) -> bool {
        let valid_range = 0u8..=23u8;
        match elem {
            "*" => true,
            _ => valid_range.contains(&elem.parse::<u8>().unwrap()),
        }
    }

    fn max(&self) -> u8 {
        23
    }
}

impl Display for Hour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(r) = &self.val {
            if r == "*" {
                return write!(f, "")
            }
            return write!(f, "past hour {r}")
        }

        let mut s = String::new();
        if let Some(r) = &self.range_start {
            s = format!("past every hour from {r} through {}", self.range_end.as_ref().unwrap());
        }

        if let Some(r) = &self.step {
            s = format!("past every {r} hour {}", &s[16..]);
        }

        write!(f, "{s}")
    }
}

impl Segment for Month {
    fn validate_range(&self, elem: &str) -> bool {
        const ALLOWED_STR: [&str; 13] = [
            "*", "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
        ];
        const ALLOWED_INT: RangeInclusive<u8> = 1u8..=12u8;

        match elem.parse::<u8>() {
            Err(_) => ALLOWED_STR.contains(&elem),
            Ok(v) => ALLOWED_INT.contains(&v),
        }
    }

    fn max(&self) -> u8 {
        12
    }
}

impl Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(r) = &self.val {
            if r == "*" {
                return write!(f, "")
            }
            return write!(f, "in month {r}")
        }

        let mut s = String::new();
        if let Some(r) = &self.range_start {
            s = format!("in every month from {r} through {}", self.range_end.as_ref().unwrap());
        }

        if let Some(r) = &self.step {
            s = format!("in every {r} month {}", &s[15..]);
        }

        write!(f, "{s}")
    }
}

impl Segment for DayOfWeek {
    fn validate_range(&self, elem: &str) -> bool {
        const ALLOWED_STR: [&str; 8] = ["*", "MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];
        const ALLOWED_INT: RangeInclusive<u8> = 0u8..=6u8;

        match elem.parse::<u8>() {
            Err(_) => ALLOWED_STR.contains(&elem),
            Ok(v) => ALLOWED_INT.contains(&v),
        }
    }

    fn max(&self) -> u8 {
        6
    }
}

impl Display for DayOfWeek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(r) = &self.val {
            if r == "*" {
                return write!(f, "")
            }
            return write!(f, "on {r}")
        }

        let mut s = String::new();
        if let Some(r) = &self.range_start {
            s = format!("on every day-of-week from {r} through {}", self.range_end.as_ref().unwrap());
        }

        if let Some(r) = &self.step {
            s = format!("on every {r} {}", &s[9..]);
        }

        write!(f, "{s}")
    }
}

impl Segment for DayOfMonth {
    fn validate_range(&self, elem: &str) -> bool {
        const ALLOWED_INT: RangeInclusive<u8> = 1u8..=31u8;
        match elem {
            "*" => true,
            _ => ALLOWED_INT.contains(&elem.parse::<u8>().unwrap()),
        }
    }

    fn max(&self) -> u8 {
        12
    }
}

impl Display for DayOfMonth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(r) = &self.val {
            if r == "*" {
                return write!(f, "")
            }
            return write!(f, "On day-of-month {r}")
        }

        let mut s = String::new();
        if let Some(r) = &self.range_start {
            s = format!("on every day-of-month from {r} through {}", self.range_end.as_ref().unwrap());
        }

        if let Some(r) = &self.step {
            s = format!("on every {r} day-of-month {}", &s[22..]);
        }

        write!(f, "{s}")
    }
}

 
