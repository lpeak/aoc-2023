use std::{path::Path, fs::File, io::{self, BufRead}, env};


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn parse_line(line: &str) -> Vec<i64> {
    line.split(" ").map(|n| n.parse().unwrap()).collect()
}


fn compute_diff(row: &Vec<i64>) -> Vec<i64> {
    row.windows(2).map(|w| w[1] - w[0]).collect()
}

fn is_zeros(row: &Vec<i64>) -> bool {
    for number in row {
        if *number != 0 {
            return false;
        }
    }
    true
}

fn process_row_part1(row: Vec<i64>) -> i64 {
    let mut sequences: Vec<Vec<i64>> = Vec::new();
    let mut current_row = row;

    while !is_zeros(&current_row) {
        let new_row = compute_diff(&current_row);
        sequences.push(current_row);
        current_row = new_row;
    }

    sequences.iter().rev().fold(0, |acc, row| acc + row[row.len() - 1])
}

fn process_row_part2(row: Vec<i64>) -> i64 {
    let mut sequences: Vec<Vec<i64>> = Vec::new();
    let mut current_row = row;

    while !is_zeros(&current_row) {
        let new_row = compute_diff(&current_row);
        sequences.push(current_row);
        current_row = new_row;
    }

    sequences.iter().rev().fold(0, |acc, row| row[0] - acc)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = read_lines(filename).unwrap();

    // let total_sum: i64 = lines.map(|l| process_row_part1(parse_line(&l.unwrap()))).sum();
    let total_sum: i64 = lines.map(|l| process_row_part2(parse_line(&l.unwrap()))).sum();
    println!("Sum is {total_sum}");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_diff() {
        let input = vec![1, 3, 6, 10, 15, 21];
        let expected = vec![2, 3, 4, 5, 6];
        assert_eq!(compute_diff(&input), expected)
    }
}