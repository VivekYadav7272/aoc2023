mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod utils;

fn main() {
    // std::fs::write("day8.txt", utils::fetch_problem(8).as_bytes()).expect("couldn't write to file");
    dbg!(day8::level2(include_str!("../day8.txt")));
}
