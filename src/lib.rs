pub trait Validator {
    fn valid_values(&self) -> [char; 4] {
        ['*', ',', '-', '/']
    }
    fn validate_range(&self, item: &str) -> bool;
    fn value(&self) -> &str;

    fn _validate(&self, expr: &str) -> bool {
        let (has_slash , has_dash) = (expr.contains('/'), expr.contains('-'));

        // match (has_slash, has_dash) {
        //     (true, true) => (),
        //     (false, true) => (),
        //     (true, false) => (),
        //     (false, false) => (),
        // };

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

            if range_start == "*" || step_start == "*" || step_end == "*" {
                return false;
            }

            let all_range_valid = self.validate_range(range_start) && self.validate_range(step_start) && self.validate_range(step_end);

            match all_range_valid {
                false => return false,
                true => {
                    let range_start = range_start.parse::<u8>().unwrap();
                    let step_start = step_start.parse::<u8>().unwrap();
                    let _step_end = step_end.parse::<u8>().unwrap();
        
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
            return step_end != "*" && 
                   self.validate_range(step_start) && 
                   self.validate_range(step_end);
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
            let all_range_valid = self.validate_range(range_start) && self.validate_range(range_end);

            match all_range_valid {
                false => return false,
                true => {
                    let range_start = range_start.parse::<u8>().unwrap();
                    let range_end = range_end.parse::<u8>().unwrap();
                    return range_end >= range_start;
                }
            };
        }
        return self.validate_range(expr);
    }

    fn validate(&self) -> bool {
        let expr = self.value();

        if expr.contains(',') {
            return expr.split(',')
                        .map(|e| self._validate(e))
                        .all(|res| res == true )
            
        }
        self._validate(expr)
    }

}

#[derive(Debug, PartialEq, Eq)]
pub struct Minutes<'a>(&'a str);

impl<'a> Minutes<'a> {
    pub fn new(val: &'a str) -> Self {
        Minutes(val)
    }
}

impl<'a> Validator for Minutes<'a> {
    fn validate_range(&self, elem: &str) -> bool {
        let valid_range = 0u8..=59u8;
        match elem {
            "*" => true,
            _ => valid_range.contains(&elem.parse::<u8>().unwrap())
        }
    }

    fn value(&self) -> &str {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hour<'a>(&'a str);

impl<'a> Hour<'a> {
    pub fn new(val: &'a str) -> Self {
        Self(val)
    }
}

impl<'a> Validator for Hour<'a> {
    fn validate_range(&self, elem: &str) -> bool {
        let valid_range = 0u8..=23u8;
        match elem {
            "*" => true,
            _ => valid_range.contains(&elem.parse::<u8>().unwrap())
        }
    }

    fn value(&self) -> &str {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DayOfMonth<'a>(&'a str);

impl<'a> DayOfMonth<'a> {
    pub fn new(val: &'a str) -> Self {
        Self(val)
    }
}

impl<'a> Validator for DayOfMonth<'a> {
    fn validate_range(&self, elem: &str) -> bool {
        let valid_range = 1u8..=31u8;
        match elem {
            "*" => true,
            _ => valid_range.contains(&elem.parse::<u8>().unwrap())
        }
    }

    fn value(&self) -> &str {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Month<'a>(&'a str);

impl<'a> Month<'a> {
    pub fn new(val: &'a str) -> Self {
        Self(val)
    }
}

impl<'a> Validator for Month<'a> {
    fn validate_range(&self, elem: &str) -> bool {
        let allowed_str = ["*", "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];
        let allowed_int = 1u8..=12u8;

        match elem.parse::<u8>() {
            Err(_) => return allowed_str.contains(&elem),
            Ok(v)  => return allowed_int.contains(&v),
        }
    }

    fn value(&self) -> &str {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DayOfWeek<'a> (&'a str);

impl<'a> DayOfWeek<'a> {
    pub fn new(val: &'a str) -> Self {
        Self(val)
    }
}

impl<'a> Validator for DayOfWeek<'a> {
    fn validate_range(&self, elem: &str) -> bool {
        let allowed_str = ["*", "MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];
        let allowed_int = 0u8..=6u8;

        match elem.parse::<u8>() {
            Err(_) =>  return allowed_str.contains(&elem),
            Ok(v)  => return allowed_int.contains(&v),
        };
    }

    fn value(&self) -> &str {
        self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CronEntry<'a> {
    pub minutes: Minutes<'a>,
    pub hour: Hour<'a>,
    pub day_of_month: DayOfMonth<'a>,
    pub month: Month<'a>,
    pub day_of_week: DayOfWeek<'a>
}

impl<'a> CronEntry<'a> {

    pub fn build(elems: &Vec<&'a str>) -> Result<Self, &'static str> {
        let minutes = Minutes::new(&elems[0]); // "0-5/2"
        let hour = Hour::new(&elems[1]);
        let day_of_month = DayOfMonth::new(&elems[2]);
        let month = Month::new(&elems[3]);
        let day_of_week = DayOfWeek::new(&elems[4]);

        if !minutes.validate() {
            return Err("Invalid value provided for minutes")
        }

        if !hour.validate() {
            return Err("Invalid value provided for hour");
        }

        if !day_of_month.validate() {
            return Err("Invalid value provided for Day of Month");
        }

        if !month.validate() {
            return Err("Invalid value provided for Month");
        }

        if !day_of_week.validate() {
            return Err("Invalid value provided for Day of Week");
        }

        Ok(Self{
            minutes,
            hour,
            day_of_month,
            month,
            day_of_week
        })
    }
}


impl<'a> Default for CronEntry<'a> {
    fn default() -> Self {
        CronEntry { minutes: Minutes(""), 
                    hour: Hour(""), 
                    day_of_month: DayOfMonth(""), 
                    month: Month(""),
                    day_of_week: DayOfWeek("")
                }
    }    
}

impl<'a> From<&'a str> for CronEntry<'a> {
    fn from(str_value: &'a str) -> Self {
        let splitted = str_value.split(" ").collect::<Vec<&'a str>>();
        if splitted.len() < 5 {
            return CronEntry::default()
        }

        if let Ok(x) = Self::build(&splitted) {
            return x;
        }
        return CronEntry::default();
    }
}
/*
    MINUTE(u8),
    HOUR(u8),
    DAY_MONTH(u8),
    MONTH(u8),
    DAY_WEEK(u),

    What's the purpose of this library?
    1- It takes a cron job expression validate it
    2- It shows the details to the user when exactly its job will be run
    3- Translates the cron expression to a human readable form
    - Eventually when that time arrives run the user program?
*/


#[cfg(test)]

mod tests {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_validate() {
        let expr = "10 4 10 FEB SUN";
        let expr = expr.split(" ").collect::<Vec<&str>>();
        let r = CronEntry::build(&expr).unwrap();

        println!("{r:?}");
        assert_eq!(r.minutes.value(), "10");
        assert_eq!(r.hour.value(), "4");
        assert_eq!(r.day_of_month.value(), "10");
        assert_eq!(r.month.value(), "FEB");
        assert_eq!(r.day_of_week.value(), "SUN");
    }

    #[test]
    fn test_parse_range() {
        let test_cases = HashMap::from([
            ("1-12", true),
            ("*-*", false),
            ("12-1", false),
            ("12-12", true),
            ("*-10", false),
            ("1-10/2", true),
        ]);

        test_cases.iter()
                  .for_each(|(case, expected)|{
                        let min = Minutes(case);
                        assert_eq!(min.validate(), *expected);
                    });
    }

    #[test]
    #[should_panic]
    fn test_invalid_range() {
        let expr = "70 4 10 JAN 3";
        let expr = expr.split(" ").collect::<Vec<&str>>();
        let r = CronEntry::build(&expr).unwrap();
        assert_eq!(r.minutes.value(), "70");
    }
}