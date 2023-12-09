use std::cmp::{max, min};
use std::{env, fs::File, path::Path};
use std::io::{self, BufRead};
use regex::Regex;
use lazy_static::lazy_static;


lazy_static! {
    static ref SEEDS_RE: Regex = Regex::new(r"^seeds: ([\d ]+)$").unwrap();
}

pub trait Range {
    fn in_range(&self, number: u64) -> bool;
    fn offset(&self, number: u64) -> u64;
    fn intersect(&self, other: &Self) -> Option<Self> where Self:Sized;
}

#[derive(Clone, Debug)]
struct BasicRange {
    range_start: u64,
    range_end: u64
}

impl Range for BasicRange {
    fn in_range(&self, number: u64) -> bool {
        number >= self.range_start
        && number < self.range_end
    }

    fn offset(&self, number: u64) -> u64 {
        number - self.range_start
    }

    fn intersect(&self, other: &Self) -> Option<Self> where Self:Sized {
        let new_start = max(self.range_start, other.range_start);
        let new_end = min(self.range_end, other.range_end);
        if new_end > new_start {
            Some(Self { range_start: new_start, range_end: new_end })
        } else {
            None
        }
    }
}

struct BasicMapping {
    origin: BasicRange,
    destination: BasicRange,
}

trait MappingCollection {
    fn get_mapped_from_origin(&self, number: u64) -> u64;
    fn map_ranges_from_origin(&self, range: Vec<BasicRange>) -> Vec<BasicRange>;
}

// #[derive(Clone)]
struct BasicMappingCollection {
    ranges: Vec<BasicMapping>,
}

impl MappingCollection for BasicMappingCollection {
    fn get_mapped_from_origin(&self, number: u64) -> u64 {
        for mapping in self.ranges.iter() {
            if mapping.origin.in_range(number) {
                return mapping.destination.range_start + mapping.origin.offset(number);
            }
        }
        number
    }
    fn map_ranges_from_origin(&self, ranges: Vec<BasicRange>) -> Vec<BasicRange> {
        let mut new_ranges_to_process = ranges.clone();
        let mut final_ranges: Vec<BasicRange> = Vec::new();
        loop {
            let next_processing: Vec<_> = new_ranges_to_process.drain(..).collect();
            'outer: for range in next_processing.iter() {
                for mapping in self.ranges.iter() {
                    if let Some(intersection) = range.intersect(&mapping.origin) {
                        let start_offset = intersection.range_start - mapping.origin.range_start;
                        let end_offset = mapping.origin.range_end - intersection.range_end;
                        final_ranges.push(BasicRange { range_start: mapping.destination.range_start + start_offset, range_end: mapping.destination.range_end - end_offset});
                        if intersection.range_start > range.range_start {
                            new_ranges_to_process.push(BasicRange { range_start: range.range_start, range_end: intersection.range_start });
                        }
                        if intersection.range_end < range.range_end {
                            new_ranges_to_process.push(BasicRange { range_start: intersection.range_end, range_end: range.range_end });
                        }
                        continue 'outer;
                    } else {
                        continue;
                    }
                }
                final_ranges.push(range.clone());
            }
            if new_ranges_to_process.is_empty() {
                break;
            }
        }
        final_ranges
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn parse_seeds(line: &str) -> Vec<u64> {
//     let seed_numbers: Vec<u64> = SEEDS_RE.captures(line).unwrap().get(1).unwrap().as_str().split(" ").map(|n| n.parse().unwrap()).collect();
//     seed_numbers
// }

fn parse_seeds2(line: &str) -> Vec<BasicRange> {
    let seed_numbers: Vec<u64> = SEEDS_RE.captures(line).unwrap().get(1).unwrap().as_str().split(" ").map(|n| n.parse().unwrap()).collect();
    seed_numbers.chunks(2).map(|c| BasicRange {range_start: c[0], range_end: c[0] + c[1]}).collect()
}


fn parse_remainder(lines: &mut io::Lines<io::BufReader<File>>) -> Vec<BasicMappingCollection>{
    let mut mapping_collection: Vec<BasicMappingCollection> = Vec::new();
    let mut current_mappings: Vec<BasicMapping> = Vec::new();

    for line in lines {
        let current_line = line.unwrap();
        if current_line.ends_with("map:") {
            continue;
        }
        if current_line.is_empty() {
            mapping_collection.push(BasicMappingCollection {ranges: current_mappings.drain(..).collect()});
            continue;
        }

        let numbers: Vec<u64> = current_line.split(" ").map(|s| s.parse().unwrap()).collect();
        current_mappings.push( BasicMapping {
            origin: BasicRange { range_start: numbers[1], range_end: numbers[1] + numbers[2] },
            destination: BasicRange { range_start: numbers[0], range_end: numbers[0] + numbers[2] },
        })
    }

    if !current_mappings.is_empty() {
        mapping_collection.push(BasicMappingCollection {ranges: current_mappings.drain(..).collect()});
    }

    mapping_collection
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file_iterator = read_lines(filename).unwrap();
    // let seeds = parse_seeds(&file_iterator.next().unwrap().unwrap());
    let seeds = parse_seeds2(&file_iterator.next().unwrap().unwrap());
    file_iterator.next();

    let all_mappings: Vec<BasicMappingCollection> = parse_remainder(&mut file_iterator);

    // let minimum_last = seeds.iter().map(|s| all_mappings.iter().fold(*s, |o, mapping| mapping.get_mapped_from_origin(o))).min().unwrap();
    let minimum_last = all_mappings.iter().fold(seeds, |range, mapping| mapping.map_ranges_from_origin(range)).iter().map(|range| range.range_start).min().unwrap();
    println!("Minimum: {minimum_last}");
}
