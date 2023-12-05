use regex::Regex;
use std::{
  collections::{HashMap, HashSet},
  env, fs,
  io::Error,
};

pub fn day_five() -> Result<(), Error> {
  let args: Vec<String> = env::args().collect();
  let file_path = &args[2];

  let contents = match fs::read_to_string(file_path) {
    Ok(contents) => contents,
    Err(e) => panic!("Problem opening the file: {:?}", e),
  };

  let mut lines = contents.lines().into_iter();

  let mut active = parse_seeds_numbers(&lines.next().unwrap());

  for line in lines {
    if line.ends_with("map:") {
      println!("Previous active is {:?}", active);
      active = active
        .clone()
        .into_iter()
        .map(|(_, value)| (value, value))
        .collect();
      println!("New active is {:?}", active);
      continue;
    }

    if line.is_empty() {
      continue;
    }

    let (destination, source, length) = parse_digits_in_line(&line);
    active.clone().keys().into_iter().for_each(|key| {
      if key >= &source && key <= &(source + (length - 1)) {
        let offset = key - source;
        *active.get_mut(key).unwrap() = destination + offset;
      }
    });
  }
  let min = active.values().into_iter().min().unwrap();
  println!(
    "lowest location number that corresponds to any of the initial seed numbers: {:?}",
    min
  );

  Ok(())
}

fn parse_digits_in_line(line: &str) -> (usize, usize, usize) {
  let re = Regex::new(r"(?<destination>\d+)\s(?<source>\d+)\s(?<length>\d+)").unwrap();
  let captures = re.captures(line).unwrap();
  let source = captures
    .name("source")
    .unwrap()
    .as_str()
    .parse::<usize>()
    .unwrap();
  let destination = captures
    .name("destination")
    .unwrap()
    .as_str()
    .parse::<usize>()
    .unwrap();
  let length = captures
    .name("length")
    .unwrap()
    .as_str()
    .parse::<usize>()
    .unwrap();

  (destination, source, length)
}

fn parse_seeds_numbers(line: &str) -> HashMap<usize, usize> {
  let re = Regex::new(r"(\d+ \d+)").unwrap();
  let mut seeds: HashMap<usize, usize> = HashMap::new();
  re.find_iter(line).for_each(|m| {
    let mut start_and_length = m.as_str().split(" ").into_iter();
    let start: usize = start_and_length.next().unwrap().parse::<usize>().unwrap().to_owned();
    let length: usize = start_and_length.next().unwrap().parse::<usize>().unwrap().to_owned();
    for i in start..start+length {
      seeds.insert(i, i);
    }
  });
  seeds
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn line_is_parsed() {
    let result = parse_digits_in_line("13 37 9001");
    assert_eq!(result, (13, 37, 9001));
  }

  #[test]
  fn get_seeds() {
    let result = parse_seeds_numbers("seeds: 12 3 23 4");
    let expected: HashMap<usize, usize> = [(12, 12), (13, 13), (14,14), (23, 23), (24, 24), (25, 25), (26, 26)]
      .iter()
      .cloned()
      .collect();
    assert_eq!(result, expected);
  }
}
