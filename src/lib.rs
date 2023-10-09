
pub trait Validator {
    fn valid_values(&self) -> [char; 4] {
        ['*', ',', '-', '/']
    }
    fn validate_range(&self) -> Result<bool, &'static str>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Minutes<'a> {
    pub val: &'a str
}

impl<'a> Minutes<'a> {
    pub fn new(val: &'a str) -> Self {
        Self{
            val
        }
    }
}

impl<'a> Validator for Minutes<'a> {
    fn validate_range(&self) -> Result<bool, &'static str> {
        let int_value = match self.val.parse::<u8>() {
            Err(_) => {
                println!("{}", self.val.chars().next().unwrap());
                return Ok(self.valid_values().contains(&self.val.chars().next().unwrap()));
            },
            Ok(v)  => v
        };

        let valid_range = int_value >= 0 && int_value <= 59;
        Ok(valid_range)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hour<'a> {
    pub val: &'a str
}

impl<'a> Hour<'a> {
    pub fn new(val: &'a str) -> Self {
        Self{
            val
        }
    }
}

impl<'a> Validator for Hour<'a> {
    fn validate_range(&self) -> Result<bool, &'static str> {
        let int_value = match self.val.parse::<u8>() {
            Err(_) => {
                return Ok(self.valid_values().contains(&self.val.chars().next().unwrap()));
            },
            Ok(v)  => v
        };

        let valid_range = int_value >= 0 && int_value <=23;
        Ok(valid_range)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DayOfMonth<'a> {
    pub val: &'a str
}

impl<'a> DayOfMonth<'a> {
    pub fn new(val: &'a str) -> Self {
        Self{val}
    }
}

impl<'a> Validator for DayOfMonth<'a> {
    fn validate_range(&self) -> Result<bool, &'static str> {
        let int_value = match self.val.parse::<u8>() {
            Err(_) => {
                println!("VALUE FOR DOM {:?}", self.val.chars().next());
                return Ok(self.valid_values().contains(&self.val.chars().next().unwrap()));
            },
            Ok(v)  => v
        };

        let valid_range = int_value >= 1 && int_value <= 31;
        Ok(valid_range)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Month<'a> {
    pub val: &'a str
}

impl<'a> Month<'a> {
    pub fn new(val: &'a str) -> Self {
        Self{val}
    }
}

impl<'a> Validator for Month<'a> {
    fn validate_range(&self) -> Result<bool, &'static str> {
        let allowed_str = ["JAN", "FEV", "MAR", "AVR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"];

        let int_value = match self.val.parse::<u8>() {
            Err(_) => {
                 match allowed_str.contains(&self.val) {
                    true => return Ok(true),
                    false => return Ok(self.valid_values().contains(&self.val.chars().next().unwrap()))
                 }
            },
            Ok(v)  => v
        };

        let valid_range = int_value >= 1 && int_value <= 12;
        Ok(valid_range)
    }
}

#[derive(Debug, PartialEq, Eq)]

pub struct DayOfWeek<'a> {
    pub val: &'a str
}

impl<'a> DayOfWeek<'a> {
    pub fn new(val: &'a str) -> Self {
        Self{val}
    }
}

impl<'a> Validator for DayOfWeek<'a> {
    fn validate_range(&self) -> Result<bool, &'static str> {
        let allowed_str = ["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];

        let int_value = match self.val.parse::<u8>() {
            Err(_) => {
                match allowed_str.contains(&self.val) {
                    true => return Ok(true),
                    false => return Ok(self.valid_values().contains(&self.val.chars().next().unwrap()))
                 }
            },
            Ok(v)  => v
        };

        let valid_range = int_value >= 0 && int_value <= 6;
        Ok(valid_range)
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
        let minutes = Minutes::new(&elems[0]);
        let hour = Hour::new(&elems[1]);
        let day_of_month = DayOfMonth::new(&elems[2]);
        let month = Month::new(&elems[3]);
        let day_of_week = DayOfWeek::new(&elems[4]);
        
        if !minutes.validate_range().unwrap() {
            return Err("Invalid value provided for minutes");
        }

        if !hour.validate_range().unwrap() {
            return Err("Invalid value provided for hour");
        }

        if !day_of_month.validate_range().unwrap() {
            return Err("Invalid value provided for Day of Month");
        }

        if !month.validate_range().unwrap() {
            return Err("Invalid value provided for Month");
        }

        if !day_of_week.validate_range().unwrap() {
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
        CronEntry { minutes: Minutes{ val: "" }, 
                    hour: Hour{ val: "" }, 
                    day_of_month: DayOfMonth { val: "" }, 
                    month: Month { val: "" }, 
                    day_of_week: DayOfWeek { val: "" }
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

    #[test]
    fn test_validate() {
        let expr = "5 4 10 JAN 3";
        let expr = expr.split(" ").collect::<Vec<&str>>();
        let r = CronEntry::build(&expr).unwrap();

        assert_eq!(r.minutes.val.parse::<u8>().unwrap(), 5u8);
        assert_eq!(r.hour.val.parse::<u8>().unwrap(), 4u8);
        assert_eq!(r.day_of_month.val.parse::<u8>().unwrap(), 10);
        assert_eq!(r.month.val, "JAN");
        assert_eq!(r.day_of_week.val.parse::<u8>().unwrap(), 3u8);
    }
}