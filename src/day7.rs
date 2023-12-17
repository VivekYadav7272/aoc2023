use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
enum HandType {
    FiveKind = 6,
    FourKind = 5,
    FullHouse = 4,
    ThreeKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl HandType {
    fn from(hand: &str) -> Self {
        let mut card_freq = [0; 14];
        let max_card_freq = hand
            .chars()
            .map(|card| {
                card_freq[card_power(card)] += 1;
                // pretty sure bounds checking gonna be optimised for the second one.
                card_freq[card_power(card)]
            })
            .max()
            .expect("hand will have five cards");

        match max_card_freq {
            5 => HandType::FiveKind,
            4 => HandType::FourKind,
            3 => {
                // either FullHouse or ThreeKind.
                if card_freq.iter().filter(|&&freq| freq == 2).count() != 0 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeKind
                }
            }
            2 => {
                if card_freq.iter().filter(|&&freq| freq == 2).count() == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => panic!("max_card_freq should't have been larger than 5, as a hand consists of only 5 cards"),
        }
    }
}

impl Eq for HandType {}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        *self as usize == *other as usize
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    bid: usize,
    hand: String,
}

impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && (self.cmp_hand_str(other) == Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then_with(|| self.cmp_hand_str(other))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn from(hand: &str, bid: usize) -> Self {
        // Preprocessing the '?'s here..
        let mut freq_table = [0; 14];
        let max_card_freq = hand
            .chars()
            .map(|card| {
                freq_table[card_power(card)] += 1;
                (card, freq_table[card_power(card)])
            })
            .filter_map(|(card, freq)| (card != '?').then_some(freq))
            .max()
            .unwrap_or(0);

        if max_card_freq == 5 {
            // Type jump not possible, can only make stronger within the same type.
            // so just change it to the strongest suit.
            return Self {
                hand_type: HandType::FiveKind,
                bid,
                hand: hand.to_string(),
            };
        }

        // else, type jump is possible. Now, our best course of action
        // is to upgrade '?' to the card with the max freq.
        // However, it is possible that there are multiple cards with max_freq.
        // (Actually, the only cases are two cards with max_freq=2, or all ones)
        // So, we filter out the ones with max_freq, and then choose the ones amongst them
        // with highest card power, so that not only does this card get a type upgrade,
        // it also gets stronger in a hand_str-to-hand_str combat!

        let strongest_max_card_power = freq_table
            .into_iter()
            .rposition(|freq| freq == max_card_freq)
            .unwrap();
        // ^^ DW, this won't ever find '?' as the source of max_card_freq even if it
        // actually if it's freq == max_card_freq (even after filtering '?'s out)
        // because it has the lowest card power. Any other card is bound to be picked up first.

        let strongest_max_card = power2card(strongest_max_card_power);

        let masked_hand = hand.replace('?', strongest_max_card);

        Self {
            hand_type: HandType::from(&masked_hand),
            bid,
            hand: hand.to_string(),
        }
    }

    fn cmp_hand_str(&self, other: &Hand) -> Ordering {
        self.hand
            .chars()
            .map(card_power)
            .cmp(other.hand.chars().map(card_power))
    }
}

fn card_power(c: char) -> usize {
    match c {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => 10,
        'T' => 9,
        '9' => 8,
        '8' => 7,
        '7' => 6,
        '6' => 5,
        '5' => 4,
        '4' => 3,
        '3' => 2,
        '2' => 1,
        '?' => 0,
        _ => panic!("Card power asked for a non-card"),
    }
}

fn power2card(pow: usize) -> &'static str {
    &"?23456789TJQKA"[pow..pow + 1]
}

pub fn level1(s: &str) -> usize {
    let mut hands: Vec<Hand> = s
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<usize>().unwrap();
            Hand::from(hand, bid)
        })
        .collect();

    // println!("Before sorting: \n{hands:?}");
    hands.sort_unstable();
    // println!("After sorting: \n{hands:?}");

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}

// By far, the MOST annoying level2!!! They changed the rules of how the architecture is setup!
// Now, there needs to be a separate card_power function for the second level.
// Aand, there needs to be a different Hand::from_level2(..)
// UNLESS, I apply a hack:
// In level2, I parse out all 'J's, and replace them with '?'.
// Now, we can treat handle '?' in all the old functions as if it were a part of the equation from the start.
// Shouldn't be a problem.
// Oh and also we're lucky here we use the value of card_power as an ordinal instead of a cardinal,
// otherwise shifting up every number to accomodate '?' would've resulted in problems.

pub fn level2(s: &str) -> usize {
    let mut hands: Vec<Hand> = s
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<usize>().unwrap();
            let ret = Hand::from(&hand.replace('J', "?"), bid);
            println!("Converted from {hand} to {}", ret.hand);
            ret
        })
        .collect();

    // println!("Before sorting: \n{hands:?}");
    hands.sort();
    // println!("After sorting: \n{hands:?}");

    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum()
}
