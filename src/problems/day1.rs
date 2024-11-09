use super::Problem;

pub struct Day1;

impl Problem for Day1 {
    const PROBLEM_DAY: u32 = 1;
    fn level1(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let first_digit = line
                    .chars()
                    .find(char::is_ascii_digit)
                    .expect("didn't find first digit") as u32
                    - '0' as u32;
                let last_digit = line
                    .chars()
                    .rev()
                    .find(char::is_ascii_digit)
                    .expect("wtf if found first should definitely find second")
                    as u32
                    - '0' as u32;

                first_digit * 10 + last_digit
            })
            .sum::<u32>() as usize
    }

    fn level2(input: &str) -> usize {
        let numbers = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0",
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "0",
        ];

        input
            .lines()
            .map(|line| {
                let first_num = numbers
                    .iter()
                    .filter_map(|num| line.find(num).map(|ind| (ind, num)))
                    .min_by_key(|x| x.0)
                    .map(|x| name2num(x.1))
                    .expect("question pinky-promised me that there'll be atleast one number");
                let last_num = numbers
                    .iter()
                    .filter_map(|num| line.rfind(num).map(|ind| (ind, num)))
                    .max_by_key(|x| x.0)
                    .map(|x| name2num(x.1))
                    .expect("question pinky-promised me that there'll be atleast one number");

                first_num * 10 + last_num
            })
            .sum::<u32>() as usize
    }
}
fn name2num(name: &str) -> u32 {
    match name {
        "zero" | "0" => 0,
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => panic!("wtf"),
    }
}
