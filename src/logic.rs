use crate::types::*;
use std::str::FromStr;

fn parse_and_categorise<T: BuildableSegment>(expr: &str, seg: &T) -> Result<T, &'static str> {
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

        let range_start = match splitted_on_dash[0].parse::<u8>() {
            Err(_) => return Err("Ranges or lists of names are not allowed"),
            Ok(rg) => rg,
        };

        let range_end = match splitted_on_slash[0].parse::<u8>() {
            Err(_) => return Err("Ranges or lists of names are not allowed"),
            Ok(r) => r,
        };

        let step = match splitted_on_slash[1].parse::<u8>() {
            Err(_) => return Err("Ranges or lists of names are not allowed"),
            Ok(r) => r,
        };

        let all_range_valid = seg.validate_range(&range_start.to_string())
            && seg.validate_range(&range_end.to_string())
            && seg.validate_range(&step.to_string());

        if !all_range_valid || range_start > range_end {
            return Err("Ranges value provided invalid.");
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
            return Err("X/Y pattern failed validation");
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
        let range_start = match dash_split[0].parse::<u8>() {
            Err(_) => return Err("Range end provided should be valid int"),
            Ok(v) => v,
        };

        let range_end = match dash_split[1].parse::<u8>() {
            Err(_) => return Err("Range end provided should be valid int"),
            Ok(v) => v,
        };

        // validate that range_start and range_end are under the valid range
        let all_range_valid = seg.validate_range(&range_start.to_string())
            && seg.validate_range(&range_end.to_string());

        if !all_range_valid || range_start > range_end {
            return Err("Values provided are invalid");
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
        return Err("Validation for {expr} failed");
    }
    Ok(T::new(None, None, None, Some(expr.to_string())))
}

pub fn categorize<T: BuildableSegment>(expr: &str, seg: &T) -> Result<Vec<T>, &'static str> {
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
    type Err = &'static str;
    fn from_str(str_value: &str) -> Result<Self, Self::Err> {
        let elems = str_value.split_whitespace();
        let cron_entry = Self::build(elems)?;
        Ok(cron_entry)
    }
}

impl CronEntry {
    fn build<'a>(mut elems: impl Iterator<Item = &'a str>) -> Result<Self, &'static str> {
        let minutes = match elems.next() {
            Some(m) => {
                let min = Minutes::new();
                categorize(m, &min)?
            }
            None => return Err("Invalid or missing value provided for minutes"),
        };
        
        let hour = match elems.next() {
            Some(m) => {
                let hour = Hour::new();
                categorize(m, &hour)?
            }
            None => return Err("Invalid or missing value provided for hour"),
        };

        let month = match elems.next() {
            Some(m) => {
                let month = Month::new();
                categorize(m, &month)?
            }
            None => return Err("Invalid or missing value provided for month"),
        };

        let day_of_week = match elems.next() {
            Some(m) => {
                let dow = DayOfWeek::new();
                categorize(m, &dow)?
            }
            None => return Err("Invalid or missing value provided for day-of-week"),
        };

        let day_of_month = match elems.next() {
            Some(m) => {
                let dom = DayOfMonth::new();
                categorize(m, &dom)?
            }
            None => return Err("Invalid or missing value provided for day-of-month"),
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
