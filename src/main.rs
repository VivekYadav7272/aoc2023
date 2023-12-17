mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod utils;

fn main() {
    // std::fs::write("day7.txt", utils::fetch_problem(7).as_bytes()).expect("couldn't write to file");
    dbg!(day7::level2(include_str!("../day7.txt")));
}
