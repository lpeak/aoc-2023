use std::cmp::{max, min};
use std::{env, fs::File, path::Path};
use std::io::{self, BufRead};
use regex::Regex;
use lazy_static::lazy_static;


lazy_static! {
    static ref GAME_RE: Regex = Regex::new(r"^Game (\d+): (.*)$").unwrap();
}

type Reveal = (u32, u32, u32);
type Game = (u32, Vec<Reveal>);

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_reveal(reveal: &str) -> Reveal {
    let (mut red, mut green, mut blue): (u32, u32, u32) = (0, 0, 0);
    for cubes in reveal.split(", ") {
        let mut cubes = cubes.split(" ");
        let number: u32 = cubes.next().unwrap().parse().unwrap();
        let color = cubes.next().unwrap();
        match color {
            "red" => red = number,
            "green" => green = number,
            "blue" => blue = number,
            _ => (),
        }
    }
    (red, green, blue)
}

fn parse_game_line(line: &str) -> Game {
    let capture = GAME_RE.captures(line).unwrap();
    let game_id: u32 = capture.get(1).unwrap().as_str().parse().unwrap();
    let game_seq = capture.get(2).unwrap().as_str();

    let parsed_seq: Vec<Reveal> = game_seq.split("; ").map(parse_reveal).collect();
    (game_id, parsed_seq)
}

fn max_over_reveals(reveals: Vec<Reveal>) -> Reveal {
    reveals.iter().fold((0, 0, 0), |(r1, g1, b1), (r2, g2, b2)| (max(r1, *r2), max(g1, *g2), max(b1, *b2)))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let maxed_games = read_lines(filename).unwrap().map(|l| parse_game_line(&l.unwrap())).map(|(game_id, reveals)| (game_id, max_over_reveals(reveals)));
    // let game_id_sum: u32 = maxed_games.filter(|(_, (r, g, b))| *r <= 12 && *g <= 13 && *b <= 14).map(|(game_id, _)| game_id).sum();
    // println!("Total sum is: {game_id_sum}")
    let power_sum: u32 = maxed_games.map(|(_, (r, g, b))| r * g * b).sum();
    println!("Power sum is: {power_sum}")
}
