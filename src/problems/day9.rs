use crate::utils::parse_stuff;

use super::Problem;

pub struct Day9;
impl Problem for Day9 {
    const PROBLEM_DAY: u32 = 9;

    fn level1(s: &str) -> usize {
        s.lines()
            .map(|line| {
                let nums: Vec<i32> = parse_stuff(line).collect();
                calculate_prediction_level1(nums)
            })
            .sum::<i32>() as usize
    }

    fn level2(s: &str) -> usize {
        s.lines()
            .map(|line| {
                let nums: Vec<i32> = parse_stuff(line).collect();
                calculate_prediction_level2(nums)
            })
            .sum::<i32>() as usize
    }
}

fn calculate_prediction_level1(nums: Vec<i32>) -> i32 {
    if nums.iter().all(|&num| num == 0) {
        return 0;
    }

    let next_pred = calculate_prediction_level1(nums.windows(2).map(|x| x[1] - x[0]).collect());

    nums.last()
        .expect("the `if` condn traps the zero elem case")
        + next_pred
}

fn calculate_prediction_level2(nums: Vec<i32>) -> i32 {
    if nums.iter().all(|&num| num == 0) {
        return 0;
    }

    let next_pred = calculate_prediction_level2(nums.windows(2).map(|x| x[1] - x[0]).collect());

    nums.first()
        .expect("the `if` condn traps the zero elem case")
        - next_pred
}
