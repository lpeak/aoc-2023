use std::collections::HashMap;
use std::{env, fs::File, path::Path};
use std::io::{self, BufRead};
use regex::Regex;
use lazy_static::lazy_static;


// lazy_static! {
//     static ref FIRST_DIGIT_RE: Regex = Regex::new(r"^[^\d]*(\d).*$").unwrap();
//     static ref LAST_DIGIT_RE: Regex = Regex::new(r"^.*(\d)[^\d]*$").unwrap();
// }


lazy_static! {
    static ref FIRST_DIGIT_RE: Regex = Regex::new(r"^(?U:.*)(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    static ref LAST_DIGIT_RE: Regex = Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine)(?U:.*)$").unwrap();
    static ref DIGITS_MAP: HashMap<&'static str, &'static str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn extract_digits(line: &str) -> usize {
    let mut first_digit = FIRST_DIGIT_RE.captures(line).unwrap().get(1).unwrap().as_str();
    let mut last_digit = LAST_DIGIT_RE.captures(line).unwrap().get(1).unwrap().as_str();
    if let Some(number) = DIGITS_MAP.get(first_digit) {
        first_digit = number;
    }
    if let Some(number) = DIGITS_MAP.get(last_digit) {
        last_digit = number;
    }
    let number = first_digit.to_owned() + last_digit;
    number.parse::<usize>().unwrap()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let digit_iter = read_lines(filename).unwrap().map(|l| extract_digits(&l.unwrap()));
    let result: usize = digit_iter.fold(0, |a, b| a + b);
    println!("total sum is {}", result)
}
