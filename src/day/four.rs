use std::{
  collections::{HashMap, HashSet},
  env, fs,
  io::Error,
};

use regex::Regex;

pub fn day_four() -> Result<(), Error> {
  let line_re =
    Regex::new(r"Card\s+(?<card_id>\d{1,}):\s+(?<left>[\d\s]*)\s+\|\s+(?<right>[\d\s]*)").unwrap();
  let digit_re = Regex::new(r"(\d{1,})+").unwrap();

  let args: Vec<String> = env::args().collect();
  let file_path = &args[2];

  let contents = match fs::read_to_string(file_path) {
    Ok(contents) => contents,
    Err(e) => panic!("Problem opening the file: {:?}", e),
  };

  let mut cards: HashMap<isize, isize> = HashMap::new();
  let mut sum: isize = 0;
  for (_, line) in contents.lines().enumerate() {
    let captured_line = line_re.captures(line).unwrap();
    let game_id = captured_line
      .name("card_id")
      .unwrap()
      .as_str()
      .parse::<isize>()
      .unwrap();
    cards.insert(game_id, 1);
  }

  for (_, line) in contents.lines().enumerate() {
    let captured_line = line_re.captures(line).unwrap();
    let game_id = captured_line
      .name("card_id")
      .unwrap()
      .as_str()
      .parse::<isize>()
      .unwrap();

    let left_side_digits: Vec<isize> = digit_re
      .find_iter(captured_line.name("left").unwrap().as_str())
      .map(|capture| capture.as_str().parse::<isize>().unwrap())
      .collect();
    let right_side_digits: Vec<isize> = digit_re
      .find_iter(captured_line.name("right").unwrap().as_str())
      .map(|capture| capture.as_str().parse::<isize>().unwrap())
      .collect();

    let intersection: isize = left_side_digits
      .clone()
      .into_iter()
      .collect::<HashSet<isize>>()
      .intersection(
        &right_side_digits
          .clone()
          .into_iter()
          .collect::<HashSet<isize>>(),
      )
      .count()
      .try_into()
      .unwrap();

    println!("from {} to {}", game_id + 1, game_id + 1 + intersection);
    for i in game_id + 1..game_id + 1 + intersection {
      if cards.contains_key(&i) {
        *cards.get_mut(&i).unwrap() += *cards.get_mut(&game_id).unwrap();
      }
    }

  }
  let sum: isize = cards
    .into_iter()
    .map(|(_, value)| value)
    .into_iter()
    .sum();
  println!("sum {:?}",sum);

  Ok(())
}
