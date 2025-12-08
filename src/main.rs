use std::env;

use crate::days::day1::{day1, day1p2};

mod days;
mod util;

fn dispatch(day_str: &str){
    match day_str {
        "day1" => day1(),
        "day1p2" => day1p2(),
        _ => panic!("not binded")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(value) => dispatch(value),
        None => panic!("not binded")
    }
}
