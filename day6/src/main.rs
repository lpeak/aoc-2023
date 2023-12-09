// use std::iter::zip;
use std::{env, fs::File, path::Path};
use std::io::{self, BufRead};


struct Race {
    time: u64,
    distance: u64,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn parse_inputs(lines: &mut io::Lines<io::BufReader<File>>) -> Vec<Race> {
//     let time_line = lines.next().unwrap().unwrap();
//     assert!(time_line.starts_with("Time:"));
//     let time_numbers: Vec<u64> = time_line.split(" ").skip(1).filter_map(|s| s.parse().ok()).collect();

//     let distance_line = lines.next().unwrap().unwrap();
//     assert!(distance_line.starts_with("Distance:"));
//     let distance_numbers: Vec<u64> = distance_line.split(" ").skip(1).filter_map(|s| s.parse().ok()).collect();
//     zip(time_numbers, distance_numbers).map(|(t, d)| Race {time: t, distance: d}).collect()
// }

fn parse_inputs2(lines: &mut io::Lines<io::BufReader<File>>) -> Race {
    let time_line = lines.next().unwrap().unwrap();
    assert!(time_line.starts_with("Time:"));
    let time_number: u64 = time_line.split(" ").skip(1).filter(|s| !s.is_empty()).map(|s| s.to_string()).collect::<Vec<String>>().join("").parse().unwrap();

    let distance_line = lines.next().unwrap().unwrap();
    assert!(distance_line.starts_with("Distance:"));
    let distance_number: u64 = distance_line.split(" ").skip(1).filter(|s| !s.is_empty()).map(|s| s.to_string()).collect::<Vec<String>>().join("").parse().unwrap();

    Race { time: time_number, distance: distance_number }
}

fn solve_poly2(race: &Race) -> (u64, u64) {
    // It's just a 2nd degree polynomial
    let time = race.time as f64;
    let dist = (race.distance + 1) as f64;  // We need to beat the record

    let det = time.powi(2) - 4.0 * dist;
    (
        ((time - det.sqrt()) / 2.).ceil() as u64,
        ((time + det.sqrt()) / 2.).floor() as u64,
    )
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file_iterator = read_lines(filename).unwrap();
    // let races = parse_inputs(&mut file_iterator);

    // let prod_possibilities: u64 = races.iter().map(solve_poly2).map(|(l, r)| r - l + 1).product();
    // println!("Product of possibilities: {prod_possibilities}");

    let (l, r) = solve_poly2(&parse_inputs2(&mut file_iterator));
    let possibilities = r - l + 1;
    println!("Possibilities: {possibilities}");
}
