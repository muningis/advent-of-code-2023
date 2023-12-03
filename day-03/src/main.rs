use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Error;

fn find_number(line: &str, position: usize) -> (usize, usize) {
  let digit_re = Regex::new(r"[0-9]").unwrap();

  let mut start_position = position;
  let mut end_position = position;

  while start_position > 0
    && digit_re.is_match(
      line
        .chars()
        .nth(start_position - 1)
        .unwrap()
        .to_string()
        .as_str(),
    )
  {
    start_position -= 1;
  }

  while end_position < line.len() - 1
    && digit_re.is_match(
      line
        .chars()
        .nth(end_position + 1)
        .unwrap()
        .to_string()
        .as_str(),
    )
  {
    end_position += 1;
  }

  (start_position, end_position+1)
}

fn handle_line(
  found_numbers: &mut HashMap<(usize, usize, usize), isize>,
  line: &str,
  x: usize,
  y: usize,
) -> (bool, isize) {
  let (start_position, end_position) = find_number(line, x);
  let number = line[start_position..end_position]
    .parse::<isize>()
    .unwrap();
  let should_insert = !found_numbers.contains_key(&(y, start_position, end_position));
  if should_insert {
    found_numbers.insert((y, start_position, end_position), number);
  }
  (should_insert, number)
}

fn main() -> Result<(), Error> {
  let digit_re = Regex::new(r"\d").unwrap();
  let special_symbol_re = Regex::new(r"[^a-zA-Z0-9\.]").unwrap();
  let gear_re = Regex::new(r"\*").unwrap();

  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];

  let contents = match fs::read_to_string(file_path) {
    Ok(contents) => contents,
    Err(e) => panic!("Problem opening the file: {:?}", e),
  };

  let lines: Vec<&str> = contents.lines().collect();
  let height = contents.lines().into_iter().count();
  let width = contents.lines().into_iter().next().unwrap().len();

  let mut schematic: Vec<Vec<&str>> = vec![Vec::new(); height];

  for (y, line) in lines.iter().enumerate() {
    let columns: Vec<&str> = line.split("").filter(|x| !x.is_empty()).collect();
    schematic[y] = vec![""; width];
    for (x, column) in columns.iter().enumerate() {
      schematic[y][x] = column;
    }
  }

  /**
   * key (row, start_position, end_position)
   */
  let mut found_numbers: HashMap<(usize, usize, usize), isize> = HashMap::new();
  let mut adjacent_numbers_ratios_sum = 0;

  for y in 0..height - 1 {
    for x in 0..width - 1 {
      let is_symbol = special_symbol_re.is_match(schematic[y][x]);
      let is_gear = gear_re.is_match(schematic[y][x]);
      if !is_symbol {
        continue;
      };
      let mut adjacent_numbers_count = 0;
      let mut adjacent_numbers_ratio = 1;

      if x > 0 {
        let west = schematic[y][x - 1];
        if digit_re.is_match(west) {
          let (inserted, value) = handle_line(&mut found_numbers, &schematic[y].join(""), x - 1, y);
          if is_gear && inserted { println!("found gear for x {} y {} value {}", x, y, value);
            adjacent_numbers_count += 1;
            adjacent_numbers_ratio *= value;
          }
        }
      }
      if x < width - 1 {
        let east = schematic[y][x + 1];
        if digit_re.is_match(east) {
          let (inserted, value) = handle_line(&mut found_numbers, &schematic[y].join(""), x + 1, y);
          if is_gear && inserted { println!("found gear for x {} y {} value {}", x, y, value);
            adjacent_numbers_count += 1;
            adjacent_numbers_ratio *= value;
          }
        }
      }
      if y > 0 {
        let north = schematic[y - 1][x];
        if digit_re.is_match(north) {
          let (inserted, value) = handle_line(&mut found_numbers, &schematic[y - 1].join(""), x, y - 1);
          if is_gear && inserted { println!("found gear for x {} y {} value {}", x, y, value);
            adjacent_numbers_count += 1;
            adjacent_numbers_ratio *= value;
          }
        }
        if x > 0 {
          let northwest = schematic[y - 1][x - 1];
          if digit_re.is_match(northwest) {
            let (inserted, value) = handle_line(&mut found_numbers, &schematic[y - 1].join(""), x - 1, y - 1);
            if is_gear && inserted { println!("found gear for x {} y {} value {}", x, y, value);
              adjacent_numbers_count += 1;
              adjacent_numbers_ratio *= value;
            }
          }
        }
        if x < width - 1 {
          let northeast = schematic[y - 1][x + 1];
          if digit_re.is_match(northeast) {
            let (inserted, value) = handle_line(&mut found_numbers, &schematic[y - 1].join(""), x + 1, y - 1);
            if is_gear && inserted { println!("found gear for x {} y {} value {}", x, y, value);
              adjacent_numbers_count += 1;
              adjacent_numbers_ratio *= value;
            }
          }
        }
      }
      if y < height - 1 {
        let south = schematic[y + 1][x];
        if digit_re.is_match(south) {
          let (inserted, value) = handle_line(&mut found_numbers, &schematic[y + 1].join(""), x, y + 1);
          if is_gear && inserted { println!("found gear for x {} y {} value {}", x, y, value);
            adjacent_numbers_count += 1;
            adjacent_numbers_ratio *= value;
          }
        }
        if x > 0 {
          let southhwest = schematic[y + 1][x - 1];
          if digit_re.is_match(southhwest) {
            let (inserted, value) = handle_line(&mut found_numbers, &schematic[y + 1].join(""), x - 1, y + 1);
            if is_gear && inserted { println!("found gear for x {} y {} value {}", x, y, value);
              adjacent_numbers_count += 1;
              adjacent_numbers_ratio *= value;
            }
          }
        }
        if x < width - 1 {
          let southheast = schematic[y + 1][x + 1];
          if digit_re.is_match(southheast) {
            let (inserted, value) = handle_line(&mut found_numbers, &schematic[y + 1].join(""), x + 1, y + 1);
            if is_gear && inserted { println!("found gear for x {} y {} value {}", x, y, value);
              adjacent_numbers_count += 1;
              adjacent_numbers_ratio *= value;
            }
          }
        }
      }
      
      if adjacent_numbers_count == 2 { adjacent_numbers_ratios_sum += adjacent_numbers_ratio }
    }
  }

  let sum: isize = found_numbers
    .into_iter()
    .map(|(_, value)| value)
    .into_iter()
    .sum();
  println!("sum: {:?}", sum);
  println!("gears sum: {:?}", adjacent_numbers_ratios_sum);

  Ok(())
}
