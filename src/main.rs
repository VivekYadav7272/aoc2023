mod day1;
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
    // std::fs::write("day9.txt", utils::fetch_problem(9).as_bytes()).expect("couldn't write to file");
    dbg!(day9::level1(include_str!("../day9.txt")));
}
