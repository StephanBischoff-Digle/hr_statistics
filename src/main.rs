use clap::{App, Arg};

use std::collections::HashMap;

mod common;
mod day0;
mod day1;

fn main() -> Result<(), common::ProgramError> {
    let matches = App::new("Hackerrank 10 days of Statistics")
        .version("0.1")
        .author("Stephan Bischoff <stephan.bischoff.hg@googlemail.com>")
        .about("Runns the challanges for each day")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(true)
                .help("Runns challenges for the specified day"),
        )
        .get_matches();

    type FnRun = fn() -> Result<(), common::ProgramError>;
    let lookup: HashMap<&str, FnRun> = [("0", day0::run as FnRun), ("1", day1::run as FnRun)]
        .iter()
        .cloned()
        .collect();

    if let Some(day) = matches.value_of("day") {
        match lookup.get(&day) {
            Some(f) => f()?,
            None => println!("Nothing implemented for day {}!", day),
        };
    } else {
        println!("No day was specified!");
    }

    Ok(())
}
