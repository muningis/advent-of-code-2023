use std::env;
use std::fs;
use std::io::Error;
use regex::Regex;

fn main() -> Result<(), Error> {
    let digits_re = Regex::new(r"\d").unwrap();

    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    let lines = contents.split("\n");
    let mut numbers: Vec<i64> = Vec::new();

    for line in lines {
        let captures = digits_re.find_iter(line);
        let digits: Vec<&str> = captures.map(|capture| capture
            .as_str()
        ).collect();
        let first = digits[0];
        let last = digits[digits.len() - 1];
        let number = format!("{}{}", first, last).parse::<i64>().unwrap();
        numbers.push(number);
    }

    let sum: i64 = numbers.iter().sum();

    print!("Sum is: {}", sum);
    Ok(())
}
