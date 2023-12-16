mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod utils;

fn main() {
    // std::fs::write("day6.txt", utils::fetch_problem(6).as_bytes()).expect("couldn't write to file");
    dbg!(day6::level2(include_str!("../day6.txt")));
}
