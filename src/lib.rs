use std::str::FromStr;
use std::ops::RangeInclusive;

pub trait Segment {
    fn validate_range(&self, item: &str) -> bool;
    fn value(&self) -> &String;
}

pub fn validate<T: Segment>(segment: &T) -> bool {
    let expr = segment.value();
    let f = |v: &str| segment.validate_range(v);

    if expr.contains(',') {
        return expr.split(',')
                    .map(|e| {
                        return validate_segment(e, f);
                    })
                    .all(|res| res == true );
        
    }
    validate_segment(expr, f)
}

fn validate_segment(expr: &str, validate_range: impl Fn(&str) -> bool) -> bool {
    let (has_slash , has_dash) = (expr.contains('/'), expr.contains('-'));

    if expr == "*" {
        return true;
    }
    //- X-Y/Z, where Y >= X and X Y Z inside the valid range of values of the type
    // 5-10/3
    if has_slash && has_dash {
        let splitted_on_dash= expr.split('-').collect::<Vec<&str>>();
        let splitted_on_slash = splitted_on_dash[1].split('/').collect::<Vec<&str>>();

        let range_start = splitted_on_dash[0];
        let step_start = splitted_on_slash[0];
        let step_end = splitted_on_slash[1];
        
        // None of the values can be * in this case
        if range_start == "*" || step_start == "*" || step_end == "*" {
            return false;
        }

        let all_range_valid = validate_range(range_start) && 
                                    validate_range(step_start) && 
                                    validate_range(step_end);

        match all_range_valid {
            false => return false,
            true => {
                let range_start = match range_start.parse::<u8>() {
                    Err(_) => return true,
                    Ok(e) => e,
                };
                let step_start = match step_start.parse::<u8>() {
                    Err(_) => return true,
                    Ok(e) => e,
                };
                let _step_end = match step_end.parse::<u8>() {
                    Err(_) => return true,
                    Ok(e) => e,
                };
    
                if range_start >= step_start {
                    println!("Step start is greater than range start {step_start} > {range_start}");
                    return false;
                }
                return true
            }
        }
    }

    // - X/Y, where X, Y inside the valid range of values of the type
    if has_slash && !has_dash {
        let slash_sp= expr.split('/').collect::<Vec<&str>>();
        let step_start = slash_sp[0];
        let step_end = slash_sp[1];
        // Note: step_start can be *
        // validate that step_start and step_end are under the valid range        
        return step_end != "*" && validate_range(step_start) && validate_range(step_end);
    }

    // - X-Y, where Y >= X and X, Y inside the valid range of values of the type
    if has_dash && !has_slash {
        let dash_split = expr.split('-').collect::<Vec<&str>>();
        let range_start = dash_split[0];
        let range_end = dash_split[1];

        if range_start == "*" || range_end == "*" {
            return false;
        }   
        // validate that range_start and range_end are under the valid range
        let all_range_valid = validate_range(range_start) && validate_range(range_end);

        match all_range_valid {
            false => return false,
            true => {
                let range_start = match range_start.parse::<u8>() {
                    Ok(v) => v,
                    Err(_) => return true,
                };
                let range_end = match range_end.parse::<u8>() {
                    Ok(v) => v,
                    Err(_) => return true,
                };

                return range_end >= range_start;
            }
        };
    }
    return validate_range(expr);
}

#[derive(Debug, PartialEq, Eq)]
pub struct Minutes(String);

impl Minutes {
    pub fn new(val: String) -> Self {
        Minutes(val)
    }
}

impl Segment for Minutes {

    fn validate_range(&self, elem: &str) -> bool {
        let valid_range = 0u8..=59u8;
        match elem {
            "*" => true,
            _ => valid_range.contains(&elem.parse::<u8>().unwrap())
        }
    }

    fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hour(String);

impl Hour {
    pub fn new(val: String) -> Self {
        Self(val)
    }
}

impl Segment for Hour {

    fn validate_range(&self, elem: &str) -> bool {
        let valid_range = 0u8..=23u8;
        match elem {
            "*" => true,
            _ => valid_range.contains(&elem.parse::<u8>().unwrap())
        }
    }

    fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DayOfMonth(String);

impl DayOfMonth {
    pub fn new(val: String) -> Self {
        Self(val)
    }
}

impl Segment for DayOfMonth {

    fn validate_range(&self, elem: &str) -> bool {
        let valid_range = 1u8..=31u8;
        match elem {
            "*" => true,
            _ => valid_range.contains(&elem.parse::<u8>().unwrap())
        }
    }

    fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Month(String);

impl Month {
    pub fn new(val: String) -> Self {
        Self(val)
    }
}

impl Segment for Month {

    fn validate_range(&self, elem: &str) -> bool {
        const ALLOWED_STR: [&str; 13] = ["*", "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];
        const ALLOWED_INT: RangeInclusive<u8> = 1u8..=12u8;

        match elem.parse::<u8>() {
            Err(_) => return ALLOWED_STR.contains(&elem),
            Ok(v)  => return ALLOWED_INT.contains(&v),
        }
    }

    fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DayOfWeek(String);

impl DayOfWeek {
    pub fn new(val: String) -> Self {
        Self(val)
    }
}

impl Segment for DayOfWeek {

    fn validate_range(&self, elem: &str) -> bool {
        const ALLOWED_STR: [&str; 8] = ["*", "MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];
        const ALLOWED_INT: RangeInclusive<u8> = 0u8..=6u8;

        match elem.parse::<u8>() {
            Err(_) =>  return ALLOWED_STR.contains(&elem),
            Ok(v)  => return ALLOWED_INT.contains(&v),
        };
    }

    fn value(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CronEntry {
    pub minutes: Minutes,
    pub hour: Hour,
    pub day_of_month: DayOfMonth,
    pub month: Month,
    pub day_of_week: DayOfWeek,
}

impl CronEntry {

    fn build<'a>(mut elems: impl Iterator<Item = &'a str>) -> Result<Self, &'static str> {
        let minutes = match elems.next() {
            Some(m) => Minutes::new(m.to_string()),
            None => return Err("Invalid or missing value provided for minutes")
        };

        let hour = match elems.next() {
            Some(m) => Hour::new(m.to_string()),
            None => return Err("Invalid or missing value provided for hour")
        };

        let day_of_month =  match elems.next() {
            Some(m) => DayOfMonth::new(m.to_string()),
            None => return Err("Invalid value provided for day of month")
        };

        let month =  match elems.next() {
            Some(m) => Month::new(m.to_string()),
            None => return Err("Invalid or missing value provided for month")
        };

        let day_of_week =  match elems.next() {
            Some(m) => DayOfWeek::new(m.to_string()),
            None => return Err("Invalid or missing value provided for day of week")
        };

        Ok(Self{
            minutes,
            hour,
            day_of_month,
            month,
            day_of_week
        })
    }  

}


impl FromStr for CronEntry {
    type Err = &'static str;

    fn from_str(str_value: &str) -> Result<Self, Self::Err> {
        let elems = str_value.split(" ");
        let cron_entry = Self::build(elems)?;
        Ok(cron_entry)
    }
}

#[cfg(test)]

mod tests {

    use super::*;
    use std::collections::HashMap;

    #[test]

    fn test_to_string() {
        // When there's 
        let cases = HashMap::from([
            ("* * * * *", "At every minute"),
        ]);
    }

    #[test]
    fn test_validate() {
        let expr = "10 4 10 JAN-FEB/MAR SUN";
        let r = CronEntry::from_str(expr).unwrap();

        println!("{r:?}");
        assert_eq!(r.minutes.value(), "10");
        assert_eq!(r.hour.value(), "4");
        assert_eq!(r.day_of_month.value(), "10");
        assert_eq!(r.month.value(), "JAN-FEB/MAR");
        assert_eq!(r.day_of_week.value(), "SUN");
    }

    #[test]
    fn test_parse_range() {
        let test_cases = HashMap::from([
            ("1-12", true),
            ("*-*", false),
            ("*", true),
            ("12-1", false),
            ("12-12", true),
            ("*-10", false),
            ("1-10/2", true),
            ("10,11,0-12", true),
            ("10,11,12-9", false),
            ("10,11,9-12", true),
            ("19,11,9-12/3", true),
            ("19,*,9-12/3", true),
        ]);

        test_cases.into_iter()
                  .for_each(|(case, expected)|{
                        let min = Minutes(case.to_string());
                        assert_eq!(validate(&min), expected);
                    });
    }

    #[test]
    fn test_invalid_range() {
        let expr = "70 4 10 JAN 3";
        let r = CronEntry::from_str(expr).unwrap();
        assert_eq!(validate(&r.minutes) , false);
    }
}