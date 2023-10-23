mod types;
mod logic;

use logic::CronEntry;
use std::env;
use std::str::FromStr;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    let expr = &args[1];
    let s = CronEntry::from_str(expr).unwrap_or_else(|e| {
        println!("This error happened {}", e);
        std::process::exit(1);
    });

    println!("The cron expression [{}] is valid", expr);
    Ok(())
}
