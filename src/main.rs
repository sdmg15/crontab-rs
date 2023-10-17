use crontab::{CronEntry, validate};
use std::str::FromStr;
use std::env;

pub fn main() -> Result<(), Box<dyn std::error::Error>>{

    let args = env::args().collect::<Vec<String>>();
    let expr = &args[1];
    let s = CronEntry::from_str(expr).unwrap_or_else(|e| {
        println!("This error happened {}", e);
        std::process::exit(1);
    });

    match validate(&s.minutes) {
        false => println!("Invalid value provided for minutes"),
        true => ()
    }

    match validate(&s.hour) {
        false => println!("Invalid value provided for hour"),
        true => ()
    }

    match validate(&s.month) {
        false => println!("Invalid value provided for month"),
        true => ()
    }

    match validate(&s.day_of_month) {
        false => println!("Invalid value provided for day of month"),
        true => ()
    }

    match validate(&s.day_of_week) {
        false => println!("Invalid value provided for day of week"),
        true => ()
    }
    
    println!("The cron expression [{}] is valid", expr);
    Ok(())
}