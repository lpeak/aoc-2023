use std::{path::Path, fs::File, io::{self, BufRead}, env, collections::HashMap};


enum Op {
    Remove,
    Add(u32),
}

struct Instruction<'a> {
    label: &'a str,
    operation: Op
}

struct Lens {
    label: String,
    focal: u32,
}

struct LensBox {
    number: u32,
    lenses: Vec<Lens>,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn hash_char(start: u32, character: u8) -> u32 {
    ((start + character as u32) * 17) % 256
}

fn hash_string(string: &str) -> u32 {
    string.as_bytes().iter().fold(0, |acc, c| hash_char(acc, *c))
}

fn parse_input(input: &str) -> Vec<&str> {
    input.split(",").collect()
}

fn parse_input2(input: &str) -> Vec<Instruction> {
    input.split(",").map(|entry|
        if entry.contains("=") {
            let mut parts = entry.split("=");
            let label = parts.next().unwrap();
            let number: u32 = parts.next().unwrap().parse().unwrap();
            Instruction { label, operation: Op::Add(number) }
        } else {
            let mut parts = entry.split("-");
            let label = parts.next().unwrap();
            Instruction { label, operation: Op::Remove }
        }
    ).collect()
}

fn process_instruction(boxes: &mut HashMap<u32, LensBox>, instruction: &Instruction) {
    let hash_value = hash_string(instruction.label);
    match instruction.operation {
        Op::Remove => match boxes.get_mut(&hash_value) {
            Some(lbox) => {
                if let Some(index) = lbox.lenses.iter().position(|lens| lens.label == instruction.label) {
                    lbox.lenses.remove(index);
                }
            }
            None => ()
        },
        Op::Add(focal) => {
            let new_lens = Lens {label: instruction.label.to_string(), focal};
            match boxes.get_mut(&hash_value) {
                Some(lbox) => {
                    if let Some(index) = lbox.lenses.iter().position(|lens| lens.label == instruction.label) {
                        lbox.lenses[index] = new_lens;
                    } else {
                        lbox.lenses.push(new_lens)
                    }
                }
                None => {
                    boxes.insert(hash_value, LensBox { number: hash_value, lenses: vec![new_lens]} );
                }
            }
        },
    }
}

fn lens_box_score(lens_box: &LensBox) -> u32 {
    lens_box.lenses.iter().enumerate().map(|(rank, lens)| (lens_box.number + 1) * (rank as u32 + 1) * lens.focal).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut lines = read_lines(filename).unwrap();
    let first_line = lines.next().unwrap().unwrap();
    let input = parse_input(&first_line);
    let hash_sum: u32 = input.iter().map(|s|  hash_string(*s)).sum();
    println!("(Part 1) Hash sum: {hash_sum}");

    let mut boxes = HashMap::new();
    let instructions = parse_input2(&first_line);

    instructions.iter().for_each(|instruction| process_instruction(&mut boxes, instruction));
    let boxes_score: u32 = boxes.values().map(lens_box_score).sum();
    println!("(Part 2) Boxes score: {boxes_score}");
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        let result = hash_string("cm-");
        let expected = 253;
        assert_eq!(result, expected);
    }

}