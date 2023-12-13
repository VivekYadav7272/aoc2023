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

// this is a DP question, but we ain't memoisin'
pub fn level2(s: &str) {
    let card_winnings: Vec<usize> = s
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
                .map(|num_str| num_str.parse().expect("must be a frickin num"))
                .collect();

            winning_nos
                .filter(|winning_num| my_nos.contains(&winning_num))
                .count()
        })
        .collect();

    let mut total_cards_owned = card_winnings.len();

    (0..card_winnings.len())
        .for_each(|card| inc_child(card, &mut total_cards_owned, &card_winnings));

    println!("{total_cards_owned}");
}

fn inc_child(card_id: usize, total_cards_owned: &mut usize, card_winnings: &Vec<usize>) {
    let wins = card_winnings[card_id];

    (card_id + 1..=(card_id + wins).min(card_winnings.len() - 1)).for_each(|child_card| {
        *total_cards_owned += 1;
        inc_child(child_card, total_cards_owned, card_winnings);
    });
}
