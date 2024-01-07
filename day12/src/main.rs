use std::{path::Path, fs::File, io::{self, BufRead}, env, collections::HashMap};

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum SprintStatus {
    Operational,
    Broken,
    Unknown,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Validity<'a> {
    Known(bool),
    Unknown(usize, &'a[u32]),
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    springs: Vec<SprintStatus>,
    records: Vec<u32>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_spring(statuses: &str) -> Vec<SprintStatus> {
    statuses.chars().map(|c| match c {
        '.' => SprintStatus::Operational,
        '#' => SprintStatus::Broken,
        '?' => SprintStatus::Unknown,
        _ => panic!(),
    }).collect()
}

fn parse_record(record: &str) -> Vec<u32> {
    record.split(",").map(|c| c.parse().unwrap()).collect()
}

fn parse_line(line: &str) -> (Vec<SprintStatus>, Vec<u32>) {
    let result: Vec<&str> = line.split(" ").collect();
    (
        parse_spring(result[0]),
        parse_record(result[1]),
    )
}

fn parse_line2(line: &str) -> (Vec<SprintStatus>, Vec<u32>) {
    let (initial_spring, initial_record) = parse_line(line);
    let mut spring = initial_spring.clone();
    let mut record = initial_record.clone();
    for _ in 0..4 {
        spring.push(SprintStatus::Unknown);
        spring.extend(initial_spring.iter());
        record.extend(initial_record.iter())
    }
    (spring, record)
}


fn is_valid<'a, I>(status: I, record: &'a[u32]) -> Validity
where
    I: IntoIterator<Item = SprintStatus>,
{
    let mut nb_damaged = 0;
    let mut status_iterator = status.into_iter();
    let mut position = 0;
    let mut last_operational_pos = 0;

    while let Some(s) = status_iterator.next() {
        position += 1;
        match s {
            SprintStatus::Broken => {nb_damaged += 1; ()},
            SprintStatus::Operational => {
                last_operational_pos = position;
                if nb_damaged > 0 {
                    break;
                }
            },
            SprintStatus::Unknown => {
                if !record.is_empty() && nb_damaged > record[0] {
                    return Validity::Known(false);
                } else {
                    return Validity::Unknown(last_operational_pos, record);
                }
            },
        }
    }
    if record.is_empty() {
        if nb_damaged == 0 {
            return Validity::Known(true);
        } else {
            return Validity::Known(false);
        }
    } else {
        if record[0] == nb_damaged {
            match is_valid(status_iterator, &record[1..]) {
                Validity::Unknown(child_position, new_record) => return Validity::Unknown(position + child_position, new_record),
                res => return res,
            }
        } else {
            return Validity::Known(false);
        }
    }
}

fn count_arrangement(state: State, cache: &mut HashMap<State, u64>) -> u64 {
    if let Some(count) = cache.get(&state) {
        return *count;
    }

    let count = match is_valid(state.springs.clone(), &state.records[..]) {
        Validity::Known(true) => 1,
        Validity::Known(false) => 0,
        Validity::Unknown(position, new_record) => {
            let new_springs = state.springs[position..].to_vec();
            match new_springs.iter().position(|s| *s == SprintStatus::Unknown) {
                Some(index) => {
                    let mut assume_operational = new_springs.clone();
                    assume_operational[index] = SprintStatus::Operational;
                    let mut assume_broken = new_springs.clone();
                    assume_broken[index] = SprintStatus::Broken;
                    count_arrangement(State {springs: assume_operational, records: new_record.to_vec()}, cache)
                    + count_arrangement(State {springs: assume_broken, records: new_record.to_vec()}, cache)
                }
                None => panic!(),
            }
        }
    };
    cache.insert(state, count);
    count
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = read_lines(filename).unwrap();
    let mut cache: HashMap<State, u64> = HashMap::new();

    let total_arrangements: u64 = lines.map(|l| {
        // let (springs, record) = parse_line(&l.unwrap());
        let (springs, record) = parse_line2(&l.unwrap());
        count_arrangement(State { springs: springs, records: record }, &mut cache)
    }).sum();
    println!("Total arrangements: {total_arrangements}");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let (input_spring, input_record) = parse_line("#....######..#####. 1,6,5");
        assert_eq!(is_valid(input_spring, &input_record[..]), Validity::Known(true));

        let (input_spring2, input_record2) = parse_line("#....######..###### 1,6,5");
        assert_eq!(is_valid(input_spring2, &input_record2[..]), Validity::Known(false));
    }

    #[test]
    fn test_count_arrangement() {
        let (input_spring, input_record) = parse_line("?#?#?#?#?#?#?#? 1,3,1,6");
        assert_eq!(count_arrangement(input_spring, &input_record), 1);

        let (input_spring2, input_record2) = parse_line("?###???????? 3,2,1");
        assert_eq!(count_arrangement(input_spring2, &input_record2), 10);
    }
}