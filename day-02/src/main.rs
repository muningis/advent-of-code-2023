use regex::Regex;
use std::{env, fs};

const RED: i64 = 12;
const GREEN: i64 = 13;
const BLUE: i64 = 14;

fn is_valid(game: &str) -> bool {
    let game_re = Regex::new(
        r"((?<red>\d{1,} red(, )?)|(?<blue>\d{1,} blue(, )?)|(?<green>\d{1,} green(, )?))*",
    )
    .unwrap();
    let digit_re = Regex::new(r"\d{1,}").unwrap();
    let mut check = true;
    game.split("; ").for_each(|round| {
        let rolls = game_re.captures(round).unwrap();
        let red = match rolls.name("red") {
            None => 0,
            Some(red) => digit_re
                .find(red.as_str())
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap(),
        };
        let green = match rolls.name("green") {
            None => 0,
            Some(green) => digit_re
                .find(green.as_str())
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap(),
        };
        let blue = match rolls.name("blue") {
            None => 0,
            Some(blue) => digit_re
                .find(blue.as_str())
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap(),
        };
        if red > RED || green > GREEN || blue > BLUE {
            check = false;
        }
    });
    check
}

fn get_power(game: &str) -> i64 {
    let game_re = Regex::new(
        r"((?<red>\d{1,} red(, )?)|(?<blue>\d{1,} blue(, )?)|(?<green>\d{1,} green(, )?))*",
    )
    .unwrap();
    let digit_re = Regex::new(r"\d{1,}").unwrap();
    let mut min_red: i64 = 1;
    let mut min_green: i64 = 1;
    let mut min_blue: i64 = 1;

    game.split("; ").for_each(|round| {
      let rolls = game_re.captures(round).unwrap();
      let red = match rolls.name("red") {
          None => 0,
          Some(red) => digit_re
              .find(red.as_str())
              .unwrap()
              .as_str()
              .parse::<i64>()
              .unwrap(),
      };
      let green = match rolls.name("green") {
          None => 0,
          Some(green) => digit_re
              .find(green.as_str())
              .unwrap()
              .as_str()
              .parse::<i64>()
              .unwrap(),
      };
      let blue = match rolls.name("blue") {
          None => 0,
          Some(blue) => digit_re
              .find(blue.as_str())
              .unwrap()
              .as_str()
              .parse::<i64>()
              .unwrap(),
      };
      if red > min_red { min_red = red }
      if blue > min_blue { min_blue = blue }
      if green > min_green { min_green = green }
  });

    min_blue * min_green * min_red
}

fn main() {
    let data_re =
        Regex::new(r"Game (?<game_id>\d{1,}): ((\d{1,} (red(, )?|blue(, )?|green(, )?)(; )?){1,})")
            .unwrap();
    let mut valid_ids: Vec<i64> = Vec::new();
    let mut powers: Vec<i64> = Vec::new();

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let captures = data_re.captures_iter(&contents);
    captures.for_each(|capture| {
        let game_id = capture.name("game_id").unwrap().as_str();
        let game = capture.get(2).unwrap().as_str();
        powers.push(get_power(game));
        if is_valid(game) {
            valid_ids.push(game_id.parse::<i64>().unwrap());
        }
    });

    let ids_sum: i64 = valid_ids.iter().sum();
    let powers_sum: i64 = powers.iter().sum();
    println!("Sum of games IDs with possible cubes combinations: {}", ids_sum);
    println!("Sum of games powers: {}", powers_sum);
}
