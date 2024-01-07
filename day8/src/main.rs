use std::{path::Path, fs::File, io::{self, BufRead}, env};
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;


lazy_static! {
    static ref NODE_RE: Regex = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)$").unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &str, map: &mut HashMap<String, (String, String)>) {
    let capture = NODE_RE.captures(line).unwrap();
    let key = String::from(capture.get(1).unwrap().as_str());
    let left = String::from(capture.get(2).unwrap().as_str());
    let right = String::from(capture.get(3).unwrap().as_str());
    map.insert(key, (left, right));
}


// fn explore(instructions: &str, map: &HashMap<String, (String, String)>, first_node: String) -> usize {
//     let mut current_node = first_node;
//     for (i, instruction) in instructions.chars().cycle().enumerate() {
//         if current_node == "ZZZ" {
//             return i;
//         }
//         match instruction {
//             'L' => current_node = map.get(&current_node).unwrap().0.clone(),
//             'R' => current_node = map.get(&current_node).unwrap().1.clone(),
//             _ => panic!(),
//         }
//     };
//     panic!();
// }


fn explore_multiple(instructions: &str, map: &HashMap<String, (String, String)>, first_nodes: Vec<String>) -> usize {
    let mut current_nodes = first_nodes;
    for (i, instruction) in instructions.chars().cycle().enumerate() {
        if current_nodes.iter().all(|s| s.ends_with("Z")) {
            return i;
        }
        // if current_nodes.iter().any(|s| s.ends_with("Z")) {
        //     println!("{i}, {:?}", current_nodes);
        // }
        current_nodes = current_nodes.iter().map(|node|
            match instruction {
                'L' => map.get(node).unwrap().0.clone(),
                'R' => map.get(node).unwrap().1.clone(),
                _ => panic!(),
            }
        ).collect();
    };
    panic!();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut lines = read_lines(filename).unwrap();
    let instructions = lines.next().unwrap().unwrap();
    lines.next();

    let first_line = lines.next().unwrap().unwrap();
    let mut mapping: HashMap<String, (String, String)> = HashMap::new();
    parse_line(&first_line, &mut mapping);
    lines.for_each(|line| parse_line(&line.unwrap(), &mut mapping));

    // let nb_steps = explore(&instructions, &mapping, String::from("AAA"));
    lines = read_lines(filename).unwrap();
    let first_nodes: Vec<String> = lines.skip(2).map(|line| String::from(line.unwrap().split(" = ").next().unwrap())).filter(|node| node.ends_with("A")).collect();
    let nb_steps = explore_multiple(&instructions, &mapping, first_nodes);
    println!("number of steps: {nb_steps}");
}
