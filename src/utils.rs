use std::{fmt::Debug, process::Command, str::FromStr};

const COOKIE: &str = include_str!("../.env");

pub fn fetch_problem(day: u8) -> String {
    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    // Use curl command to fetch the contents and return it
    String::from_utf8_lossy(
        Command::new("curl")
            .arg("-H")
            .arg(COOKIE)
            .arg(url)
            .output()
            .expect("failed to execute curl")
            .stdout
            .as_slice(),
    )
    .to_string()
}

pub fn parse_stuff<T: FromStr>(s: &str) -> impl Iterator<Item = T> + '_
where
    <T as FromStr>::Err: Debug,
{
    s.split_whitespace().map(|x| x.parse::<T>().unwrap())
}
