use std::ops::RangeInclusive;
pub trait Segment {
    fn validate_range(&self, item: &str) -> bool;
    // fn value(&self) -> &String;
    fn max(&self) -> u8;
}

pub trait BuildableSegment: Segment {
    fn new(range_start: OptStr, range_end: OptStr, step: OptStr, val: OptStr) -> Self;
}

type OptStr = Option<String>;
#[derive(Debug, PartialEq, Eq)]
pub struct Minutes {
    pub range_start: Option<String>,
    pub range_end: Option<String>,
    pub step: Option<String>,
    pub val: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hour {
    pub range_start: Option<String>,
    pub range_end: Option<String>,
    pub step: Option<String>,
    pub val: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DayOfWeek {
    pub range_start: Option<String>,
    pub range_end: Option<String>,
    pub step: Option<String>,
    pub val: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Month {
    pub range_start: Option<String>,
    pub range_end: Option<String>,
    pub step: Option<String>,
    pub val: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DayOfMonth {
    pub range_start: OptStr,
    pub range_end: OptStr,
    pub step: OptStr,
    pub val: OptStr,
}

impl Month {
    pub fn new() -> Self {
        Self {
            range_start: None,
            range_end: None,
            step: None,
            val: None,
        }
    }
}

impl Minutes {
    pub fn new() -> Self {
        Self {
            range_start: None,
            range_end: None,
            step: None,
            val: None,
        }
    }
}

impl Hour {
    pub fn new() -> Self {
        Self {
            range_start: None,
            range_end: None,
            step: None,
            val: None,
        }
    }
}

impl DayOfWeek {
    pub fn new() -> Self {
        Self {
            range_start: None,
            range_end: None,
            step: None,
            val: None,
        }
    }
}

impl DayOfMonth {
    pub fn new() -> Self {
        Self {
            range_start: None,
            range_end: None,
            step: None,
            val: None,
        }
    }
}

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

impl BuildableSegment for Minutes {
    fn new(range_start: OptStr, range_end: OptStr, step: OptStr, val: OptStr) -> Minutes {
        Minutes {
            range_start,
            range_end,
            step,
            val,
        }
    }
}

impl BuildableSegment for Hour {
    fn new(range_start: OptStr, range_end: OptStr, step: OptStr, val: OptStr) -> Hour {
        Hour {
            range_start,
            range_end,
            step,
            val,
        }
    }
}

impl BuildableSegment for Month {
    fn new(range_start: OptStr, range_end: OptStr, step: OptStr, val: OptStr) -> Month {
        Month {
            range_start,
            range_end,
            step,
            val,
        }
    }
}

impl BuildableSegment for DayOfWeek {
    fn new(range_start: OptStr, range_end: OptStr, step: OptStr, val: OptStr) -> DayOfWeek {
        DayOfWeek {
            range_start,
            range_end,
            step,
            val,
        }
    }
}

impl BuildableSegment for DayOfMonth {
    fn new(range_start: OptStr, range_end: OptStr, step: OptStr, val: OptStr) -> DayOfMonth {
        DayOfMonth {
            range_start,
            range_end,
            step,
            val,
        }
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

impl Segment for Month {
    fn validate_range(&self, elem: &str) -> bool {
        const ALLOWED_STR: [&str; 13] = [
            "*", "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
        ];
        const ALLOWED_INT: RangeInclusive<u8> = 1u8..=12u8;

        match elem.parse::<u8>() {
            Err(_) => return ALLOWED_STR.contains(&elem),
            Ok(v) => return ALLOWED_INT.contains(&v),
        }
    }
    fn max(&self) -> u8 {
        12
    }
}

impl Segment for DayOfWeek {
    fn validate_range(&self, elem: &str) -> bool {
        const ALLOWED_STR: [&str; 8] = ["*", "MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];
        const ALLOWED_INT: RangeInclusive<u8> = 0u8..=6u8;

        match elem.parse::<u8>() {
            Err(_) => return ALLOWED_STR.contains(&elem),
            Ok(v) => return ALLOWED_INT.contains(&v),
        };
    }

    fn max(&self) -> u8 {
        6
    }
}

impl Segment for DayOfMonth {
    fn validate_range(&self, elem: &str) -> bool {
        const ALLOWED_STR: [&str; 13] = [
            "*", "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
        ];
        const ALLOWED_INT: RangeInclusive<u8> = 1u8..=12u8;

        match elem.parse::<u8>() {
            Err(_) => return ALLOWED_STR.contains(&elem),
            Ok(v) => return ALLOWED_INT.contains(&v),
        }
    }

    fn max(&self) -> u8 {
        12
    }
}
