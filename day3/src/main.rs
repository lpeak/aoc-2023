use std::{env, fs::File, path::Path};
use std::io::{self, BufRead};
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;


lazy_static! {
    static ref NUMBER_RE: Regex = Regex::new(r"(\d+)").unwrap();
    static ref SYMBOL_RE: Regex = Regex::new(r"([^\.\d])").unwrap();
}

#[derive(Clone, Copy)]
struct ParsedNumber {
    number: u32,
    x_start: i32,
    x_end: i32,
    y: i32,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct ParsedSymbol {
    symbol: char,
    x: i32,
    y: i32,
}

type GearMap = HashMap<ParsedSymbol, Vec<ParsedNumber>>;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_numbers(line: &str, line_number: i32) -> Vec<ParsedNumber> {
    let mut parsed = Vec::new();
    for capture in NUMBER_RE.captures_iter(line) {
        parsed.push(ParsedNumber {
            number: capture.get(1).unwrap().as_str().parse().unwrap(),
            x_start: capture.get(1).unwrap().start() as i32,
            x_end: capture.get(1).unwrap().end() as i32 - 1,
            y: line_number
        })
    }
    parsed
}

fn parse_symbols(line: &str, line_number: i32) -> Vec<ParsedSymbol> {
    let mut parsed = Vec::new();
    for capture in SYMBOL_RE.captures_iter(line) {
        parsed.push(ParsedSymbol {
            symbol: capture.get(1).unwrap().as_str().chars().next().unwrap(),
            x: capture.get(1).unwrap().start() as i32,
            y: line_number
        })
    }
    parsed
}

fn is_adjacent(number: &ParsedNumber, symbols: &Vec<ParsedSymbol>) -> bool {
    for symbol in symbols.iter() {
        if
            number.y >= symbol.y - 1
            && number.y <= symbol.y + 1
            && number.x_start <= symbol.x + 1
            && number.x_end >= symbol.x - 1
        {return true ;}
    }
    false
}

fn register_symbol(number: &ParsedNumber, gear_map: &mut GearMap, symbols: &Vec<ParsedSymbol>) {
    for symbol in symbols.iter() {
        if
            number.y >= symbol.y - 1
            && number.y <= symbol.y + 1
            && number.x_start <= symbol.x + 1
            && number.x_end >= symbol.x - 1
        {
            match gear_map.get_mut(&symbol) {
                Some(list) => list.push(*number),
                None => match gear_map.insert(*symbol, vec![*number]){
                    None => (),
                    Some(_) => panic!("Found value")
                },
            };
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let all_symbols: Vec<ParsedSymbol> = read_lines(filename).unwrap().enumerate().map(|(i, line)| parse_symbols(&line.unwrap(), i as i32)).flatten().collect();
    let all_numbers: Vec<ParsedNumber> = read_lines(filename).unwrap().enumerate().map(|(i, line)| parse_numbers(&line.unwrap(), i as i32)).flatten().collect();
    // let sum: u32 = all_numbers.iter().filter(|number| is_adjacent(number, &all_symbols)).map(|number| number.number).sum();
    // println!("Sum is: {sum}")
    let mut gear_map: GearMap = HashMap::new();
    let gear_symbols: Vec<ParsedSymbol> = all_symbols.into_iter().filter(|s| s.symbol == '*').collect();
    all_numbers.iter().for_each(|number| register_symbol(number, &mut gear_map, &gear_symbols));
    let gear_product_sum: u32 = gear_map.iter().filter(|(_, v)| v.len() == 2).map(|(_, v)| v[0].number * v[1].number).sum();
    println!("Gear sum is: {gear_product_sum}")
}
