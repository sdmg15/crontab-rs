mod logic;
mod types;

use clap::{arg, Command};
use logic::CronEntry;
use std::str::FromStr;

fn cli() -> Command {
    Command::new("crontab")
        .about("A simple crontab to make humans life easier")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("validate")
                .about("Check if the given cron expr is valid")
                .arg(arg!(<EXPR> "The cron expression to validate"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("astext")
                .about("Displays a potential text representation of the cron expression")
                .arg(arg!(<EXPR> "The cron expresssion"))
                .arg_required_else_help(true),
        )
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("validate", sub_matches)) => {
            let expr = sub_matches.get_one::<String>("EXPR").expect("required");
            let _r = CronEntry::from_str(expr).unwrap_or_else(|e| {
                println!("This error happened while parsing the expression: {}", e);
                std::process::exit(1);
            });
            println!("The expression {expr} is valid");
        }
        Some(("astext", sub_matches)) => {
            let expr = sub_matches.get_one::<String>("EXPR").expect("required");
            let r = CronEntry::from_str(expr).unwrap_or_else(|e| {
                println!("This error happened while parsing the expression: {}", e);
                std::process::exit(1);
            });
            println!("{r}");
        }
        _ => {
            cli().print_help().unwrap();
        }
    }
    Ok(())

    // crontab <EXPR>
}
