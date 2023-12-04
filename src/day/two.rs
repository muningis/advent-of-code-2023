use regex::{Regex, Captures};
use std::{env, fs, io::Error};

const MAX_RED: i64 = 12;
const MAX_GREEN: i64 = 13;
const MAX_BLUE: i64 = 14;

fn capture_into_i64(capture: &Captures<'_>, name: &str) -> i64 {
  let digit_re = Regex::new(r"\d{1,}").unwrap();

  match capture.name(name) {
    None => 0,
    Some(red) => digit_re
        .find(red.as_str())
        .unwrap()
        .as_str()
        .parse::<i64>()
        .unwrap(),
  }
}

fn iterate_game(game: &str) -> (bool, i64) {
    let game_re = Regex::new(
        r"((?<red>\d{1,} red(, )?)|(?<blue>\d{1,} blue(, )?)|(?<green>\d{1,} green(, )?))*",
    )
    .unwrap();

    let mut check = true;
    let mut min_red: i64 = 1;
    let mut min_green: i64 = 1;
    let mut min_blue: i64 = 1;

    game.split("; ").for_each(|round| {
        let rolls = game_re.captures(round).unwrap();
        let red = capture_into_i64(&rolls, "red");
        let green = capture_into_i64(&rolls, "green");
        let blue = capture_into_i64(&rolls, "blue");
        if red > MAX_RED || green > MAX_GREEN || blue > MAX_BLUE {
            check = false;
        }
        if red > min_red { min_red = red }
        if blue > min_blue { min_blue = blue }
        if green > min_green { min_green = green }
    });
    (check, min_blue * min_green * min_red)
}

pub fn day_two() -> Result<(), Error> {
    let data_re =
        Regex::new(r"Game (?<game_id>\d{1,}): ((\d{1,} (red(, )?|blue(, )?|green(, )?)(; )?){1,})")
            .unwrap();
    let mut valid_ids: Vec<i64> = Vec::new();
    let mut powers: Vec<i64> = Vec::new();

    let args: Vec<String> = env::args().collect();
    let file_path = &args[2];

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let captures = data_re.captures_iter(&contents);
    captures.for_each(|capture| {
        let game_id = capture.name("game_id").unwrap().as_str();
        let game = capture.get(2).unwrap().as_str();
        let (is_valid, power) = iterate_game(game);
        powers.push(power);
        if is_valid {
            valid_ids.push(game_id.parse::<i64>().unwrap());
        }
    });

    let ids_sum: i64 = valid_ids.iter().sum();
    let powers_sum: i64 = powers.iter().sum();
    println!("Sum of games IDs with possible cubes combinations: {}", ids_sum);
    println!("Sum of games powers: {}", powers_sum);

    Ok(())
}
