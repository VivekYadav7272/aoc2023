use std::str::FromStr;

#[derive(Debug, Default)]
struct GameSet {
    red: u8,
    blue: u8,
    green: u8,
}

#[derive(Debug, Default)]
struct Game {
    game_no: u8,
    sets: Vec<GameSet>,
}

impl Game {
    fn from_string(line: &str) -> Option<Game> {
        let stripped_line = line.strip_prefix("Game ")?;
        let (game_no, game_sets) = stripped_line.split_once(':')?;
        let game_no = game_no.parse::<u8>().ok()?;

        let sets = game_sets
            .split(';')
            .map(|set| {
                let mut gameset = GameSet::default();
                set.split(',')
                    .filter_map(|colour_string| {
                        let (cubes, colour) = colour_string.trim_start().split_once(' ')?;
                        let cubes = cubes.parse::<u8>().ok()?;
                        Some((cubes, colour))
                    })
                    .for_each(|(cubes, colour)| match colour {
                        "red" => gameset.red = cubes,
                        "green" => gameset.green = cubes,
                        "blue" => gameset.blue = cubes,
                        _ => panic!("Didn't expect colours other than RGB"),
                    });
                gameset
            })
            .collect();

        Some(Game { game_no, sets })
    }
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Game::from_string(line).ok_or("Failed to parse into Game struct")
    }
}

pub fn day2_level1(input: &str) {
    let (max_red_cubes, max_green_cubes, max_blue_cubes) = (12, 13, 14);

    let answer: u32 = input
        .lines()
        .map(|line| {
            let game = line.parse::<Game>().expect("Line must be a valid game");
            println!("{game:?}");
            for set in game.sets {
                if set.red > max_red_cubes
                    || set.green > max_green_cubes
                    || set.blue > max_blue_cubes
                {
                    return 0;
                }
            }
            game.game_no as u32
        })
        .sum();

    println!("{answer}");
}

pub fn day2_level2(input: &str) {
    let answer: u64 = input
        .lines()
        .map(|line| {
            let game = line.parse::<Game>().expect("Line should be a valid Game");
            let min_red_cubes = game
                .sets
                .iter()
                .max_by_key(|set| set.red)
                .expect("Must have atleast one set")
                .red;
            let min_green_cubes = game
                .sets
                .iter()
                .max_by_key(|set| set.green)
                .expect("Must have atleast one set")
                .green;
            let min_blue_cubes = game
                .sets
                .iter()
                .max_by_key(|set| set.blue)
                .expect("Must have atleast one set")
                .blue;

            min_red_cubes as u64 * min_green_cubes as u64 * min_blue_cubes as u64
        })
        .sum();

    println!("{answer}");
}
