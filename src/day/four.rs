use std::{env, fs, io::Error, collections::HashSet};

use regex::Regex;

pub fn day_four() -> Result<(), Error> {
  let line_re = Regex::new(r"Card\s+(?<card_id>\d{1,}):\s+(?<left>[\d\s]*)\s+\|\s+(?<right>[\d\s]*)").unwrap();
  let digit_re = Regex::new(r"(\d{1,})+").unwrap();

  let args: Vec<String> = env::args().collect();
  let file_path = &args[2];

  let contents = match fs::read_to_string(file_path) {
    Ok(contents) => contents,
    Err(e) => panic!("Problem opening the file: {:?}", e),
  };

  let mut sum: isize = 0;

  for (_, line) in contents.lines().enumerate() {
    let captured_line = line_re.captures(line).unwrap();
    let game_id = captured_line.name("card_id").unwrap().as_str().parse::<isize>().unwrap();
    let left_side_digits: Vec<isize> = digit_re.find_iter(captured_line.name("left").unwrap().as_str()).map(|capture| capture.as_str().parse::<isize>().unwrap()).collect();
    let right_side_digits: Vec<isize> = digit_re.find_iter(captured_line.name("right").unwrap().as_str()).map(|capture| capture.as_str().parse::<isize>().unwrap()).collect();
    
    let intersection: u32 = left_side_digits.clone().into_iter().collect::<HashSet<isize>>().intersection(
      &right_side_digits.clone().into_iter().collect::<HashSet<isize>>()
    ).count().try_into().unwrap();
    // println!("found {:?}, with  intersection {:?}", game_id, intersection);

    let base: isize = 2;
    sum += if intersection > 0 { base.pow(std::cmp::max(1,intersection)-1) } else { 0 }
  }

  println!("Sum is {}", sum);

  Ok(())
}