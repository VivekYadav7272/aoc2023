use std::process::Command;

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
