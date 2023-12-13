mod day1;
mod day2;
mod day3;
mod day4;
mod utils;

fn main() {
    // std::fs::write("day4.txt", utils::fetch_problem(4).as_bytes()).expect("couldn't write to file");
    day4::level2(include_str!("../day4.txt"));
}
