mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod utils;

fn main() {
    // std::fs::write("day5.txt", utils::fetch_problem(5).as_bytes()).expect("couldn't write to file");
    day5::level1(include_str!("../day5.txt"));
}
