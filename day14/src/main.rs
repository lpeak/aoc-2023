use std::{path::Path, fs::File, io::{self, BufRead}, env, collections::HashMap};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum RockStatus {
    Moving,
    Fixed,
    Empty,
}

type Platform = Vec<Vec<RockStatus>>;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &str) -> Vec<RockStatus> {
    line.chars().map(|c|
        match c {
            '.' => RockStatus::Empty,
            '#' => RockStatus::Fixed,
            'O' => RockStatus::Moving,
            other => panic!("Unknown symbol: '{other}'")
        }
    ).collect()
}


fn tilt_north(platform: &Platform) -> Platform {
    let mut tilted_platform = vec![vec![RockStatus::Empty; platform[0].len()]; platform.len()];
    let mut obstacle_row_indices = vec![None; platform[0].len()];

    for (y, line) in platform.iter().enumerate() {
        for (x, rock) in line.iter().enumerate() {
            match rock {
                RockStatus::Fixed => {
                    tilted_platform[y][x] = RockStatus::Fixed;
                    obstacle_row_indices[x] = Some(y);
                },
                RockStatus::Moving => {
                    let new_obstacle_pos = match obstacle_row_indices[x] {
                        Some(pos) => pos + 1,
                        None => 0,
                    };
                    tilted_platform[new_obstacle_pos][x] = RockStatus::Moving;
                    obstacle_row_indices[x] = Some(new_obstacle_pos);
                },
                RockStatus::Empty => (),
            }
        }
    }

    tilted_platform
}


fn rotate_right(platform: Platform) -> Platform {
    let width = platform[0].len();
    (0..width)
    .map(|x| platform.iter().rev().map(|elt| elt[x]).collect())
    .collect()
}

fn cycle_platform(mut platform: Platform) -> Platform {
    for _ in 0..4 {
        platform = tilt_north(&platform);
        platform = rotate_right(platform);
    }
    platform
}

fn find_cycle(original_platform: &Platform) -> (usize, usize) {
    let mut cache: HashMap<Platform, usize> = HashMap::new();
    let mut platform = original_platform.clone();

    let mut counter: usize = 0;
    loop {
        if let Some(index) = cache.get(&platform) {
            return (*index, counter - index);
        };
        cache.insert(platform.clone(), counter);
        platform = cycle_platform(platform);
        counter += 1;
    }
}

fn score_platform(platform: &Platform) -> usize {
    let length = platform.len();
    platform.iter().enumerate().map(
        |(y, line)| line.iter().map(|r| match r {RockStatus::Moving => length - y, _ => 0}).sum::<usize>()
    ).sum()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = read_lines(filename).unwrap();
    let platform: Platform = lines.map(|l| parse_line(&l.unwrap())).collect();

    let score: usize = score_platform(&tilt_north(&platform));
    println!("Weight (part1): {score}");

    let nb_cycles = 1000000000;
    let (start, loop_size) = find_cycle(&platform);
    println!("Found loop. Start: {start}. Size: {loop_size}");
    let short_cycle = start + ((nb_cycles - start) % loop_size);
    let score: usize = score_platform(&(0..short_cycle).fold(platform, |p, _| cycle_platform(p)));
    println!("Weight (part2): {score}");
}
