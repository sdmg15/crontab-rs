use crontab::CronEntry;
use std::env;
pub fn main() {

    let args = env::args().collect::<Vec<String>>();

    let expr = args[1].split(" ").collect::<Vec<&str>>();
    let s = CronEntry::from("5 0 * 8 SUNDAY");

    println!("{:?}", s);

    let res = match CronEntry::build(&expr) {
        Err(e) => {
            println!("This error occurred : {}", e);
            std::process::exit(1);
        },
        Ok(v) => v
    };

    println!("{res:?}");
}