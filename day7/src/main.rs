use std::cmp::Ordering;
use std::{env, fs::File, path::Path};
use std::io::{self, BufRead};
use lazy_static::lazy_static;
use std::collections::HashMap;
use counter::Counter;


lazy_static! {
    // static ref CARD_VALUE: HashMap<char, u32> = HashMap::from([
    //     ('2', 0),
    //     ('3', 1),
    //     ('4', 2),
    //     ('5', 3),
    //     ('6', 4),
    //     ('7', 5),
    //     ('8', 6),
    //     ('9', 7),
    //     ('T', 8),
    //     ('J', 9),
    //     ('Q', 10),
    //     ('K', 11),
    //     ('A', 12),
    // ]);
    static ref CARD_VALUE: HashMap<char, u32> = HashMap::from([
        ('J', 0),
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('Q', 10),
        ('K', 11),
        ('A', 12),
    ]);
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct ParsedHand {
    cards: Vec<u32>,
    hand_type: HandType,
}

impl Ord for ParsedHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            diff => diff,
        }
    }
}

impl PartialOrd for ParsedHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug)]
struct Play {
    hand: ParsedHand,
    bid: u64,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn parse_hand(cards: &Vec<char>) -> ParsedHand {
//     let converted_cards: Vec<u32> = cards.iter().map(|c| *CARD_VALUE.get(c).unwrap()).collect();
//     let count = cards.iter().collect::<Counter<_>>();
//     let mut nb_pair = 0;
//     let mut nb_triple = 0;
//     let mut hand_type = HandType::HighCard;

//     for (_, count) in count.most_common() {
//         if count == 5 {
//             hand_type = HandType::FiveOfAKind;
//             break;
//         }
//         if count == 4 {
//             hand_type = HandType::FourOfAKind;
//             break;
//         }
//         if count == 3 {
//             nb_triple += 1;
//         }
//         if count == 2 {
//             nb_pair += 1;
//         }
//     }

//     if hand_type == HandType::HighCard {
//         hand_type = match (nb_triple, nb_pair) {
//             (1, 1) => HandType::FullHouse,
//             (1, 0) => HandType::ThreeOfAKind,
//             (0, 2) => HandType::TwoPairs,
//             (0, 1) => HandType::OnePair,
//             (0, 0) => HandType::HighCard,
//             _ => todo!(),
//         };
//     }
//     ParsedHand { cards: converted_cards, hand_type: hand_type }
// }


fn parse_hand_joker(cards: &Vec<char>) -> ParsedHand {
    let converted_cards: Vec<u32> = cards.iter().map(|c| *CARD_VALUE.get(c).unwrap()).collect();
    let count = cards.iter().collect::<Counter<_>>();
    let mut nb_pair = 0;
    let mut nb_triple = 0;
    let mut hand_type = HandType::HighCard;

    let mut joker_count = *count.get(&'J').unwrap_or(&0);

    for (card, mut count) in count.most_common() {
        if card != &'J' {
            count += joker_count;
            joker_count = 0;
        } else {
            if joker_count == 5 {
                hand_type = HandType::FiveOfAKind;
                break;
            } else {
                continue;
            }
        }
        if count == 5 {
            hand_type = HandType::FiveOfAKind;
            break;
        }
        if count == 4 {
            hand_type = HandType::FourOfAKind;
            break;
        }
        if count == 3 {
            nb_triple += 1;
        }
        if count == 2 {
            nb_pair += 1;
        }
    }

    if hand_type == HandType::HighCard {
        hand_type = match (nb_triple, nb_pair) {
            (1, 1) => HandType::FullHouse,
            (1, 0) => HandType::ThreeOfAKind,
            (0, 2) => HandType::TwoPairs,
            (0, 1) => HandType::OnePair,
            (0, 0) => HandType::HighCard,
            _ => todo!(), Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!
        };
    }
    ParsedHand { cards: converted_cards, hand_type: hand_type }
}

fn parse_line(line: &str) -> Play {
    let split: Vec<_> = line.split(" ").collect();
    Play {
        // hand: parse_hand(&split[0].chars().collect()),
        hand: parse_hand_joker(&split[0].chars().collect()),
        bid: split[1].parse().unwrap(),
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file_iterator = read_lines(filename).unwrap();
    let mut plays: Vec<_> = file_iterator.map(|l| parse_line(&l.unwrap())).collect();
    plays.sort_by_key(|play| play.hand.clone());
    let score: u64 = plays.iter().enumerate().map(|(i, play)| ((i as u64) + 1) * play.bid).sum();
    println!("Total score: {score}");
}
