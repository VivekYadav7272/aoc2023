use std::{fmt::Debug, process::Command, str::FromStr};

pub fn fetch_problem(day: u8) -> String {
    let cookie = std::fs::read_to_string("../.env").expect(".env file should exist");

    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    // Use curl command to fetch the contents and return it
    String::from_utf8_lossy(
        Command::new("curl")
            .arg("-H")
            .arg(cookie)
            .arg(url)
            .output()
            .expect("failed to execute curl")
            .stdout
            .as_slice(),
    )
    .to_string()
}

pub fn parse_stuff<T: FromStr>(s: &str) -> impl Iterator<Item = T> + '_ + Clone
where
    <T as FromStr>::Err: Debug,
{
    s.split_whitespace().map(|x| x.parse::<T>().unwrap())
}
