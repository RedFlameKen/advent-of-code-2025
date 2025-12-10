use std::env;

use crate::days::{day1::{day1, day1p2}, day2::{day2, day2p2}, day3::{day3, day3p2}, day4::day4};

mod days;
mod util;

fn dispatch(day_str: &str){
    match day_str {
        "day1" => day1(),
        "day1p2" => day1p2(),
        "day2" => day2(),
        "day2p2" => day2p2(),
        "day3" => day3(),
        "day3p2" => day3p2(),
        "day4" => day4(),
        _ => day4()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(value) => dispatch(value),
        None => dispatch(""),
    }
}
