use std::collections::HashMap;

fn is_symbol(c: u8) -> bool {
    c != b'.' && !(c as char).is_ascii_digit()
}

pub fn day3_level1(s: &str) {
    let num_cols = s.find('\n').unwrap_or(0);
    let answer: usize = s
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let mut num = None;
            line.chars()
                // Dummy char to flush the last number if it remains
                .chain(std::iter::once('.'))
                .enumerate()
                .map(|(curr_col, c)| {
                    if c.is_ascii_digit() {
                        num = num.or(Some(curr_col));
                    } else if let Some(start_col) = num {
                        // >VVVV<
                        // >1234< These places need to be checked
                        // >^^^^<
                        //
                        // The > and < are unique to first and last digits only,
                        // V and ^ are for all digits.

                        // Now I've come up with a slightly dumbass technique, but will save me a lot of code.
                        // We obviously realised that the V and ^ can be done in one loop for all digits,
                        // we just need to handle the < and > ones separately.
                        // Buuut, if we cheat a lil and assume the middle > and < are also digits,
                        // then we can pretend that this rule applies to all digits
                        // (the rule that we just want to check V and ^)
                        // With the added caveat that to keep it generalised, we now assume that we also check ourselves
                        // to be a symbol or not (this isn't necessary for the actual digits, so wasted computation for them)
                        // but saved time for my lazy-ass which can write a whole para explaining the technique.

                        let s_bytes = s.as_bytes();
                        let drows = [0isize, -1, 1];

                        let cols_to_check = (start_col..curr_col).chain(
                            // The > and < ones that we pretend are digits,
                            // checking if they aren't out of bounds.
                            [start_col as isize - 1, curr_col as isize]
                                .into_iter()
                                .filter_map(|sussy_idx| {
                                    (0..num_cols as isize)
                                        .contains(&sussy_idx)
                                        .then(|| sussy_idx as usize)
                                }),
                        );

                        let should_add = cols_to_check.into_iter().any(|col| {
                            drows
                                .iter()
                                .filter_map(|&drow| {
                                    s_bytes
                                        .get(row.checked_add_signed(drow)? * (num_cols + 1) + col)
                                        .copied()
                                })
                                .any(is_symbol)
                        });

                        num = None;

                        if should_add {
                            return line[start_col..curr_col]
                                .parse::<usize>()
                                .expect("mf should be a number");
                        }
                    }
                    0
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{answer}");
}

pub fn day3_level2(s: &str) {
    let num_cols = s.find('\n').unwrap_or(0);
    // key: coord, value: (count, product of parts that encountered it)
    let mut hashmapuh = HashMap::<usize, (usize, usize)>::new();
    s.lines().enumerate().for_each(|(row, line)| {
        let mut num = None;
        line.chars()
            // Dummy char to flush the last number if it remains
            .chain(std::iter::once('.'))
            .enumerate()
            .for_each(|(curr_col, c)| {
                if c.is_ascii_digit() {
                    num = num.or(Some(curr_col));
                } else if let Some(start_col) = num {
                    // >VVVV<
                    // >1234< These places need to be checked
                    // >^^^^<
                    //
                    // The > and < are unique to first and last digits only,
                    // V and ^ are for all digits.

                    // Now I've come up with a slightly dumbass technique, but will save me a lot of code.
                    // We obviously realised that the V and ^ can be done in one loop for all digits,
                    // we just need to handle the < and > ones separately.
                    // Buuut, if we cheat a lil and assume the middle > and < are also digits,
                    // then we can pretend that this rule applies to all digits
                    // (the rule that we just want to check V and ^)
                    // With the added caveat that to keep it generalised, we now assume that we also check ourselves
                    // to be a symbol or not (this isn't necessary for the actual digits, so wasted computation for them)
                    // but saved time for my lazy-ass which can write a whole para explaining the technique.

                    let s_bytes = s.as_bytes();
                    let drows = [0isize, -1, 1];

                    let cols_to_check = (start_col..curr_col).chain(
                        // The > and < ones that we pretend are digits,
                        // checking if they aren't out of bounds.
                        [start_col as isize - 1, curr_col as isize]
                            .into_iter()
                            .filter_map(|sussy_idx| {
                                (0..num_cols as isize)
                                    .contains(&sussy_idx)
                                    .then_some(sussy_idx as usize)
                            }),
                    );

                    let parsed_num = line[start_col..curr_col]
                        .parse::<usize>()
                        .expect("mf should be usize");

                    cols_to_check.into_iter().for_each(|col| {
                        drows
                            .iter()
                            .filter_map(|&drow| {
                                let ind = row.checked_add_signed(drow)? * (num_cols + 1) + col;
                                s_bytes.get(ind).map(|&neighbour| (ind, neighbour))
                            })
                            .for_each(|(ind, neighbour)| {
                                if neighbour == b'*' {
                                    hashmapuh
                                        .entry(ind)
                                        .and_modify(|(count, product)| {
                                            *count += 1;
                                            *product *= parsed_num;
                                        })
                                        .or_insert((1, parsed_num));
                                }
                            })
                    });

                    num = None;
                }
            })
    });

    // By this time, all *'s have got their popularity scores set in.

    let answer: usize = hashmapuh
        .values()
        .filter_map(|&(count, product)| (count == 2).then_some(product))
        .sum();

    println!("{answer}");
}
