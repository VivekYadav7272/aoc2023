use crate::utils::parse_stuff;

pub fn level1(s: &str) -> usize {
    s.lines()
        .map(|line| {
            let nums: Vec<i32> = parse_stuff(line).collect();
            calculate_prediction(nums)
        })
        .sum::<i32>() as usize
}

fn calculate_prediction(nums: Vec<i32>) -> i32 {
    if nums.iter().all(|&num| num == 0) {
        return 0;
    }

    let next_pred = calculate_prediction(nums.windows(2).map(|x| x[1] - x[0]).collect());

    nums.last()
        .expect("the `if` condn traps the zero elem case")
        + next_pred
}
