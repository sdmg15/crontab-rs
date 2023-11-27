use crate::types::*;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Sub;
use std::{num::ParseIntError, str::FromStr};

use ParseError::*;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("Invalid value provided for integer type")]
    InvalidInteger(#[from] ParseIntError),
    #[error("Invalid value provided for ranges")]
    InvalidRangeValue,
    #[error("Pattern validation failed")]
    FailedPatternValidation,
}

fn parse_and_categorise<T: Segment>(expr: &str, seg: &T) -> Result<T, ParseError> {
    let (has_slash, has_dash) = (expr.contains('/'), expr.contains('-'));

    if expr == "*" {
        let r = T::new(None, None, None, Some("*".to_string()));
        return Ok(r);
    }

    //- X-Y/Z, where Y >= X and X Y Z inside the valid range of values of the type
    // 5-10/3
    if has_slash && has_dash {
        let splitted_on_dash = expr.split('-').collect::<Vec<&str>>();
        let splitted_on_slash = splitted_on_dash[1].split('/').collect::<Vec<&str>>();

        let range_start = splitted_on_dash[0].parse::<u8>().map_err(InvalidInteger)?;
        let range_end = splitted_on_slash[0].parse::<u8>().map_err(InvalidInteger)?;
        let step = splitted_on_slash[1].parse::<u8>().map_err(InvalidInteger)?;

        let all_range_valid = seg.validate_range(&range_start.to_string())
            && seg.validate_range(&range_end.to_string())
            && seg.validate_range(&step.to_string());

        if !all_range_valid || range_start > range_end {
            return Err(InvalidRangeValue);
        }

        let elem = T::new(
            Some(range_start.to_string()),
            Some(range_end.to_string()),
            Some(step.to_string()),
            None,
        );
        return Ok(elem);
    }

    // - X/Y, where X, Y inside the valid range of values of the type
    if has_slash && !has_dash {
        let slash_sp = expr.split('/').collect::<Vec<&str>>();
        let (range_start, range_end, step) = (slash_sp[0], seg.max(), slash_sp[1]);
        // Note: step_start can be *
        // validate that step_start and step_end are under the valid range
        let valid_expr = step != "*" && seg.validate_range(range_start) && seg.validate_range(step);

        if !valid_expr {
            return Err(FailedPatternValidation);
        }

        let r = T::new(
            Some(range_start.to_string()),
            Some(range_end.to_string()),
            Some(step.to_string()),
            None,
        );
        return Ok(r);
    }

    // - X-Y, where Y >= X and X, Y inside the valid range of values of the type
    if has_dash && !has_slash {
        let dash_split = expr.split('-').collect::<Vec<&str>>();
        let range_start = dash_split[0].parse::<u8>().map_err(InvalidInteger)?;
        let range_end = dash_split[1].parse::<u8>().map_err(InvalidInteger)?;

        // validate that range_start and range_end are under the valid range
        let all_range_valid = seg.validate_range(&range_start.to_string())
            && seg.validate_range(&range_end.to_string());

        if !all_range_valid || range_start > range_end {
            return Err(InvalidRangeValue);
        }

        let r = T::new(
            Some(range_start.to_string()),
            Some(range_end.to_string()),
            None,
            None,
        );
        return Ok(r);
    }
    // Single value once here
    if !seg.validate_range(expr) {
        return Err(FailedPatternValidation);
    }
    Ok(T::new(None, None, None, Some(expr.to_string())))
}

pub fn categorize<T: Segment>(expr: &str, seg: &T) -> Result<Vec<T>, ParseError> {
    let mut res: Vec<T> = Vec::new();
    for exp in expr.split(',') {
        res.push(parse_and_categorise(exp, seg)?);
    }
    Ok(res)
}

#[derive(Debug, PartialEq, Eq)]
pub struct CronEntry {
    pub minutes: Vec<Minutes>,
    pub hour: Vec<Hour>,
    pub day_of_month: Vec<DayOfMonth>,
    pub month: Vec<Month>,
    pub day_of_week: Vec<DayOfWeek>,
}

impl FromStr for CronEntry {
    type Err = ParseError;
    fn from_str(str_value: &str) -> Result<Self, Self::Err> {
        let elems = str_value.split_whitespace();
        let cron_entry = Self::build(elems)?;
        Ok(cron_entry)
    }
}

impl CronEntry {
    fn build<'a>(mut elems: impl Iterator<Item = &'a str>) -> Result<Self, ParseError> {
        let minutes = match elems.next() {
            Some(m) => {
                let min = Minutes::default();
                categorize(m, &min)?
            }
            None => return Err(InvalidRangeValue),
        };

        let hour = match elems.next() {
            Some(m) => {
                let hour = Hour::default();
                categorize(m, &hour)?
            }
            None => return Err(InvalidRangeValue),
        };

        let day_of_month = match elems.next() {
            Some(m) => {
                let dom = DayOfMonth::default();
                categorize(m, &dom)?
            }
            None => return Err(InvalidRangeValue),
        };

        let month = match elems.next() {
            Some(m) => {
                let month = Month::default();
                categorize(m, &month)?
            }
            None => return Err(InvalidRangeValue),
        };

        let day_of_week = match elems.next() {
            Some(m) => {
                let dow = DayOfWeek::default();
                categorize(m, &dow)?
            }
            None => return Err(InvalidRangeValue),
        };

        Ok(Self {
            minutes,
            hour,
            month,
            day_of_week,
            day_of_month,
        })
    }
}

impl Display for CronEntry {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        fn to_str<T>(e: &Vec<Common<T>>) -> String
        where
            Common<T>: Display,
        {
            e.iter()
                .enumerate()
                .map(|p| {
                    let s = format!("{}", p.1).to_string();
                    if e.len() > 1 {
                        if p.0 == e.len().sub(1) {
                            return format!("and {}", s.to_lowercase());
                        }
                        if p.0 == 0 {
                            return format!("{}, ", p.1);
                        } else {
                            return format!("{}, ", s.to_lowercase());
                        }
                    }
                    format!("{}", p.1)
                })
                .collect::<Vec<String>>()
                .join("")
        }

        let min = to_str(&self.minutes);
        let hour = to_str(&self.hour);
        let dom: String = to_str(&self.day_of_month);
        let month = to_str(&self.month);
        let dow = to_str(&self.day_of_week);

        if dow.is_empty() {
            return write!(f, "{min} {hour} {dom} {month}");
        }

        write!(f, "{min} {hour} {dom} {dow} {month}")
    }
}
