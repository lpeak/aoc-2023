use std::{path::Path, fs::File, io::{self, BufRead}, env, iter::zip};


//#[derive(Clone, Hash)]
type PatternLine = Vec<char>;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn line_diff(line1: &PatternLine, line2: &PatternLine) -> u32 {
    zip(line1, line2).map(|(c1, c2)| if c1 == c2 { 0 } else { 1 }).sum()
}

fn reflection_index(lines: &Vec<PatternLine>) -> Option<usize> {
    let mut lines_acc: Vec<PatternLine> = Vec::new();
    let mut line_iter = lines.iter();
    lines_acc.push(line_iter.next().unwrap().clone());

    for index in 1..lines.len() {
        let is_reflection = zip(line_iter.clone(), lines_acc.iter().rev()).all(|(l, r)| l == r);
        if is_reflection {
            return Some(index);
        }
        lines_acc.push(line_iter.next().unwrap().clone());
    }
    None
}

fn reflection_index_smudge(lines: &Vec<PatternLine>) -> Option<usize> {
    let mut lines_acc: Vec<PatternLine> = Vec::new();
    let mut line_iter = lines.iter();
    lines_acc.push(line_iter.next().unwrap().clone());

    for index in 1..lines.len() {
        let smudge_number = zip(line_iter.clone(), lines_acc.iter().rev()).fold(0, |acc, (l1, l2)| acc + line_diff(l1, l2));
        if smudge_number == 1 {
            return Some(index);
        }
        lines_acc.push(line_iter.next().unwrap().clone());
    }
    None
}

fn transpose_input(input: &Vec<PatternLine>) -> Vec<PatternLine> {
    (0..input[0].len())
    .map(|col_index| input.iter().map(|row| row[col_index]).collect())
    .collect()
}


fn parse_input(lines: io::Lines<io::BufReader<File>>) -> Vec<Vec<PatternLine>> {
    let mut current_pattern: Vec<PatternLine> = Vec::new();
    let mut patterns: Vec<Vec<PatternLine>> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            patterns.push(current_pattern.drain(..).collect());
        } else {
            current_pattern.push(line.chars().collect());
        }
    }

    if !current_pattern.is_empty() {
        patterns.push(current_pattern.drain(..).collect());
    }

    patterns
}

fn unwrap_index(i: Option<usize>) -> u32 {
    match i {
        Some(index) => index as u32,
        None => 0,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = read_lines(filename).unwrap();

    let patterns = parse_input(lines);
    let sum: u32 = patterns.iter()
        // .map(|p| 100 * unwrap_index(reflection_index(p)) + unwrap_index(reflection_index(&transpose_input(p))))
        .map(|p| 100 * unwrap_index(reflection_index_smudge(p)) + unwrap_index(reflection_index_smudge(&transpose_input(p))))
        .sum();
    println!("Sum: {sum}");
}
