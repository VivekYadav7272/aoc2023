mod day1;
mod day2;
mod day3;
mod utils;

fn main() {
    // std::fs::write("../../day3.txt", utils::fetch_problem(3).as_bytes())
    //     .expect("couldn't write to file");
    day3::day3_level2(include_str!("../day3.txt"));
}
