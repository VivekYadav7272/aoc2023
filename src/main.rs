mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

fn main() {
    use day10 as curr_day;
    use std::fs;
    const CURR_DAY: u8 = 10;

    // fs::write(
    //     format!("day{CURR_DAY}.txt"),
    //     utils::fetch_problem(CURR_DAY).as_bytes(),
    // )
    // .expect("couldn't write to file");

    dbg!(curr_day::level1(
        &fs::read_to_string(format!("day{CURR_DAY}.txt")).expect("couldn't open input file")
    ));
}
