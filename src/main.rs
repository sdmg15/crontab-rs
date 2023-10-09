use crontab::{CronEntry};

pub fn main() {

    let expr = "5 0 * 8 *";
    let expr = expr.split(" ").collect::<Vec<&str>>();
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