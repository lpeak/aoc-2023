use std::{env, fs::File, path::Path};
use std::io::{self, BufRead};
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashSet;


lazy_static! {
    static ref CARD_RE: Regex = Regex::new(r"^Card +(\d+): ([\d ]+) \| ([\d ]+)$").unwrap();
}

// #[derive(Clone)]
struct ParsedCard {
    card_number: u32,
    winning_numbers: Vec<u32>,
    registered_numbers: Vec<u32>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_card(line: &str) -> ParsedCard {
    fn parse_numbers(number_list: &str) -> Vec<u32>{
        number_list.split(" ").filter(|s| !s.is_empty()).map(|substring| substring.parse::<u32>().unwrap()).collect()
    }
    let captured = CARD_RE.captures(line).unwrap();
    ParsedCard {
        card_number: captured.get(1).unwrap().as_str().parse().unwrap(),
        winning_numbers: parse_numbers(captured.get(2).unwrap().as_str()),
        registered_numbers: parse_numbers(captured.get(3).unwrap().as_str()),
    }
}


fn count_card(card: &ParsedCard) -> u32 {
    let winning: HashSet<u32> = HashSet::from_iter(card.winning_numbers.clone());
    let registered: HashSet<u32> = HashSet::from_iter(card.registered_numbers.clone());
    winning.intersection(&registered).count() as u32
}

fn score_card(card: &ParsedCard) -> u32 {
    let nb_intersect = count_card(card);
    if nb_intersect > 0 {
        1 << (nb_intersect - 1)
    } else {
        0
    }
}

fn increment_next(counters: &mut Vec<u32>, position: usize, number: u32) {
    for i in 0..(number as usize) {
        counters[position + i + 1] += counters[position];
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let parsed: Vec<ParsedCard> = read_lines(filename).unwrap().map(|line| parse_card(&line.unwrap())).collect();
    let mut counters: Vec<u32> = vec![1; parsed.len()];
    // let result: u32 = parsed.iter().map(score_card).sum();
    // println!("Total score: {result}");
    parsed.iter().enumerate().for_each(|(i, card)| increment_next(&mut counters, i, count_card(card)));
    let result: u32 = counters.iter().sum();
    println!("Total cards: {result}");
}
