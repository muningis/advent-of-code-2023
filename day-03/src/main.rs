use std::{env, fs};

use regex::Regex;

fn main() {
  let digit_re = Regex::new(r"(\d+)").unwrap();
  let special_symbol_re = Regex::new(r"[^a-zA-Z0-9\.]").unwrap();

  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];

  let contents = match fs::read_to_string(file_path) {
    Ok(contents) => contents,
    Err(e) => panic!("Problem opening the file: {:?}", e),
  };

  let height: usize = contents.lines().into_iter().count();
  let width: usize = contents.lines().into_iter().next().unwrap().len();

  let mut sum: isize = 0;

  let lines: Vec<&str> = contents.lines().collect();
  for (y, line) in lines.clone().into_iter().enumerate() {
    for m in digit_re.find_iter(line) {
      let compare_range_start = if m.start() > 0 { m.start() - 1 } else { 0 };
      let compare_range_end = if m.end() < width - 1 {
        m.end() + 1
      } else {
        width - 1
      };
      if compare_range_start > 0
        && special_symbol_re.is_match(&line[compare_range_start..compare_range_start+1])
        || compare_range_end < width - 1
          && special_symbol_re.is_match(&line[compare_range_end-1..compare_range_end])
      {
        sum += m.as_str().parse::<isize>().unwrap();
      } else if y > 0
        && special_symbol_re.is_match(&lines[y - 1][compare_range_start..compare_range_end])
      {
        sum += m.as_str().parse::<isize>().unwrap();
      } else if y < height - 1
        && special_symbol_re.is_match(&lines[y + 1][compare_range_start..compare_range_end])
      {
        sum += m.as_str().parse::<isize>().unwrap();
      }
    }
  }

  println!("Sum is {}", sum);
}
