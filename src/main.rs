#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    /*
    let start = Instant::now();
    day1::solve();
    day2::solve();
    day3::solve();
    day4::solve();
    day5::solve();
    day6::solve();

    let elapsed_time = start.elapsed().as_micros();
    println!("Process took {} macroseconds", elapsed_time);
    */
    day6::fancy_terminal();
}
