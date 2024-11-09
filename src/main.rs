use problems::{Problem, ProblemV2};

pub mod problems;
mod utils;

fn run_problem(problem: &mut impl ProblemV2) {
    let problem_text = std::fs::read_to_string(format!("day{}.txt", problem.get_problem_day()))
        .expect("couldn't open input file");
    dbg!(problem.level1(problem_text.as_str()));
    dbg!(problem.level2(problem_text.as_str()));
}

fn fetch_problem<T: Problem>(_: &T) {
    std::fs::write(
        format!("day{}.txt", T::PROBLEM_DAY),
        utils::fetch_problem(T::PROBLEM_DAY as u8).as_bytes(),
    )
    .expect("couldn't write to file");
}

fn main() {
    let mut curr_problem = problems::day10::Day10;
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 1 && args[1] == "fetch" {
        fetch_problem(&mut curr_problem);
    } else {
        run_problem(&mut curr_problem);
    }
}
