use std::{env, io::Error};
mod day;

fn main() -> Result<(), Error> {
  let args: Vec<String> = env::args().collect();

  let day: &str = &args[1].as_str();
  let _file_path = &args[2];

  let _ = match day {
    "1" => day::one::day_one(),
    "2" => day::two::day_two(),
    "3" => day::three::day_three(),
    "4" => day::four::day_four(),
    _ => panic!("Day is not implemented"),
  };

  Ok(())
}
