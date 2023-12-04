
use regex::Regex;
use std::env;
use std::fs;
use std::io::Error;

pub fn day_one() -> Result<(), Error> {
  let digits_re = Regex::new(r"\d").unwrap();

  let args: Vec<String> = env::args().collect();
  let file_path = &args[2];

  let contents = match fs::read_to_string(file_path) {
    Ok(contents) => contents,
    Err(e) => panic!("Problem opening the file: {:?}", e),
  };

  let lines = contents.split("\n");
  let mut sum: i64 = 0;

  for line in lines {
    let transformed_line = line
    .replace("one", "o1e")
    .replace("two", "t2o")
    .replace("three", "thr3e")
    .replace("four", "f4ur")
    .replace("five", "f5ve")
    .replace("six", "s6x")
    .replace("seven", "s7ven")
    .replace("eight", "e8ght")
    .replace("nine", "n9ne");

    let captures = digits_re.find_iter(transformed_line.as_str());
    let numbers: Vec<&str> = captures.map(|capture| capture.as_str()).collect();
    println!("{:?}", numbers);
    let first =  numbers[0];
    let last = numbers[numbers.len() - 1];
    println!("{:?} -> {:?} ir {:?} yra {:?}", line, first, last,format!("{}{}", first, last));
    let number = format!("{}{}", first, last).parse::<i64>().unwrap();
    sum += number;
  }


  print!("Sum is: {}", sum);
  Ok(())
}
