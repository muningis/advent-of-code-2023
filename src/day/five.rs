use regex::Regex;
use std::{collections::HashSet, env, fs, io::Error, cmp};

pub fn day_five() -> Result<(), Error> {
  let args: Vec<String> = env::args().collect();
  let file_path = &args[2];

  let contents = match fs::read_to_string(file_path) {
    Ok(contents) => contents,
    Err(e) => panic!("Problem opening the file: {:?}", e),
  };

  let mut lines = contents.lines().into_iter();

  // let mut active = parse_seeds_ranges(&lines.next().unwrap());

  // for line in lines {
  //   if line.ends_with("map:") {
  //     println!("Previous active is {:?}", active);
  //     active = active
  //       .clone()
  //       .into_iter()
  //       .map(|(_, value)| (value, value))
  //       .collect();
  //     println!("New active is {:?}", active);
  //     continue;
  //   }

  //   if line.is_empty() {
  //     continue;
  //   }

  //   let (destination, source, length) = parse_digits_in_line(&line);
  //   active.clone().keys().into_iter().for_each(|key| {
  //     if key >= &source && key <= &(source + (length - 1)) {
  //       let offset = key - source;
  //       *active.get_mut(key).unwrap() = destination + offset;
  //     }
  //   });
  // }
  // let min = active.values().into_iter().min().unwrap();
  // println!(
  //   "lowest location number that corresponds to any of the initial seed numbers: {:?}",
  //   min
  // );

  Ok(())
}

fn parse_source_range_and_destination_offset(line: &str) -> ((isize, isize), isize) {
  let re = Regex::new(r"(?<destination>\d+)\s(?<source>\d+)\s(?<length>\d+)").unwrap();
  let captures = re.captures(line).unwrap();
  let source = captures
    .name("source")
    .unwrap()
    .as_str()
    .parse::<isize>()
    .unwrap();
  let destination = captures
    .name("destination")
    .unwrap()
    .as_str()
    .parse::<isize>()
    .unwrap();
  let length = captures
    .name("length")
    .unwrap()
    .as_str()
    .parse::<isize>()
    .unwrap();

  ((source, source + length - 1), destination - source)
}

fn parse_seeds_ranges(line: &str) -> HashSet<(usize, usize)> {
  let re = Regex::new(r"(\d+ \d+)").unwrap();
  let mut seeds_ranges: HashSet<(usize, usize)> = HashSet::new();
  re.find_iter(line).for_each(|m| {
    let mut start_and_length = m.as_str().split(" ").into_iter();
    let start: usize = start_and_length
      .next()
      .unwrap()
      .parse::<usize>()
      .unwrap()
      .to_owned();
    let length: usize = start_and_length
      .next()
      .unwrap()
      .parse::<usize>()
      .unwrap()
      .to_owned();
    seeds_ranges.insert((start, start + length - 1));
  });
  seeds_ranges
}

fn get_intersections_and_leftovers(
  left: HashSet<(usize, usize)>,
  right: HashSet<(usize, usize)>,
) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>) {
  let mut intersections: HashSet<(usize, usize)> = HashSet::new();
  let mut leftovers: HashSet<(usize, usize)> = HashSet::new();

  left.into_iter().for_each(|(left_min, left_max)| {
    let did_find = right.clone().into_iter().find(|(right_min, right_max)| {
      &left_min >= right_min && &left_max <= right_max
    });

    if did_find.is_some() {
      let (right_min, right_max) = did_find.unwrap();

      let outer_left_min = cmp::min(left_min, right_min);
      let intersection_min = cmp::max(left_min, right_min);
      let intersection_max = cmp::min(left_max, right_max);
      let outer_right_max = cmp::max(left_max, right_max);

      intersections.insert((intersection_min,intersection_max));
      leftovers.insert(( outer_left_min, intersection_min-1 ));
      leftovers.insert(( intersection_max+1, outer_right_max ));
    } else {
      leftovers.insert(( left_min, left_max ));
    }
  });


  (intersections, leftovers)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn source_range_and_destination_offset_is_parsed_and_calculated() {
    let result = parse_source_range_and_destination_offset("713612345 1290834 9823745");
    assert_eq!(result, ((1_290_834, 11_114_578), 712_321_511));

    let result = parse_source_range_and_destination_offset("1028374 189234 12934");
    assert_eq!(result, ((189_234, 202_167), 839_140));
  }

  #[test]
  fn get_seeds_ranges() {
    let result = parse_seeds_ranges("seeds: 90812735 12340123 12345 1351235 9823475 23475");
    let expected: HashSet<(usize, usize)> = [
      (90_812_735, 103_152_857),
      (12_345, 1_363_579),
      (9_823_475, 9_846_949),
    ]
    .iter()
    .cloned()
    .collect();
    assert_eq!(result, expected);
  }

  #[test]
  fn successfully_get_intersections_and_leftovers() {
    let left: HashSet<(usize, usize)> = [(1, 10), (20, 35)].iter().cloned().collect();

    let right: HashSet<(usize, usize)> = [(3, 14), (19, 25)].iter().cloned().collect();

    let result = get_intersections_and_leftovers(left, right);
    let expected_intersection: HashSet<(usize, usize)> =
      [(3, 10), (20, 25)].iter().cloned().collect();
    let expected_leftovers: HashSet<(usize, usize)> = [(1,2), (11,14), (19,19), (26,35)].iter().cloned().collect();
    assert_eq!(result, (expected_intersection, expected_leftovers));
  }
}
