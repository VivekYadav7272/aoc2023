pub mod day1;
pub mod day10;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

pub trait Problem {
    const PROBLEM_DAY: u32;
    fn level1(s: &str) -> usize;
    fn level2(s: &str) -> usize;
}

pub trait ProblemV2 {
    const PROBLEM_DAY: u32;
    fn get_problem_day(&self) -> u32 {
        Self::PROBLEM_DAY
    }
    fn level1(&mut self, s: &str) -> usize;
    fn level2(&mut self, s: &str) -> usize;
}

impl<T> ProblemV2 for T
where
    T: Problem + Default,
{
    const PROBLEM_DAY: u32 = T::PROBLEM_DAY;

    fn level1(&mut self, s: &str) -> usize {
        T::level1(s)
    }

    fn level2(&mut self, s: &str) -> usize {
        T::level2(s)
    }
}
