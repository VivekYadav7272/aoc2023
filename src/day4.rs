pub fn level1(s: &str) {
    let answer = s
        .lines()
        .map(|line| {
            let (_card_id, rest) = line.split_once(":").expect("colon must be present");
            let (winning_nos, my_nos) = rest.split_once("|").expect("| must be present");
            let winning_nos = winning_nos
                .trim()
                .split_whitespace()
                .map(|num_str| num_str.parse::<usize>().expect("must be a frickin num"));

            let my_nos: Vec<usize> = my_nos
                .trim()
                .split_whitespace()
                .map(|num_str| num_str.parse::<usize>().expect("must be a frickin num"))
                .collect();

            let matches = winning_nos
                .filter(|winning_num| my_nos.contains(&winning_num))
                .count();

            if matches == 0 {
                0
            } else {
                1usize << (matches - 1)
            }
        })
        .sum::<usize>();

    println!("{answer}");
}
