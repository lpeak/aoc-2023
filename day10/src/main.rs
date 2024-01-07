use std::{path::Path, fs::File, io::{self, BufRead}, env, collections::HashSet};
use petgraph::Undirected;
use petgraph::prelude::GraphMap;
use petgraph::algo::all_simple_paths;



type GridGraph = GraphMap<(usize, usize), (), Undirected>;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn connected_left_to_right(left: char, right: char) -> bool {
    match left {
        '-' | 'L' | 'F' | 'S' => match right {
            '-' | 'J' | '7' | 'S' => true,
            _ => false
        }
        _ => false
    }
}


fn connected_up_to_down(up: char, down: char) -> bool {
    match up {
        '|' | '7' | 'F' | 'S' => match down {
            '|' | 'L' | 'J' | 'S' => true,
            _ => false
        }
        _ => false
    }
}


fn parse_grid(lines: &Vec<&str>) -> GridGraph {
    let mut graph = GridGraph::new();

    for (y, rows) in lines.windows(2).enumerate() {
        let (up, down) = (rows[0], rows[1]);
        let vertical_pairs = up.chars().zip(down.chars()).enumerate().filter(|(_, (u, d))| connected_up_to_down(*u, *d)).map(|(x, _)| (x, y));
        vertical_pairs.for_each(|(x, y)| {graph.add_edge((x, y), (x, y + 1), ()) ;} );
        let horizontal_pairs = up.as_bytes().windows(2).enumerate().filter(|(_x, slice)| connected_left_to_right(slice[0] as char, slice[1] as char)).map(|(x, _)| (x, y));
        horizontal_pairs.for_each(|(x, y)| {graph.add_edge((x, y), (x + 1, y), ());});
    }
    // last row
    let y_max = lines.len() - 1;
    let last_line_pairs = lines[y_max].as_bytes().windows(2).enumerate().filter(|(_x, slice)| connected_left_to_right(slice[0] as char, slice[1] as char)).map(|(x, _)| (x, y_max));
    last_line_pairs.for_each(|(x, y)| {graph.add_edge((x, y), (x + 1, y), ());});
    graph
}

fn find_starting_point(lines: &Vec<&str>) -> Option<(usize, usize)> {
    for (y, row) in lines.iter().enumerate() {
        for (x, char) in row.char_indices() {
            if char == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}


fn find_loop_in_graph(graph: &GridGraph, start: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    // let edges: Vec<_> = graph.all_edges().collect();
    // println!("Edges: {:?}", edges);
    for neighbor in graph.neighbors(start) {
        // println!("N: {:?}", neighbor);
        if let Some(way) = all_simple_paths::<Vec<_>, _>(&graph, start, neighbor, 1, None).next() {
            // println!("Path: {:?}", way);
            return Some(way);
        }
    }
    None
}

fn trace_right(loop_set: HashSet<(usize, usize)>, lines: &Vec<&str>) -> Vec<Vec<Option<u32>>> {
    let traced_grid: Vec<Vec<Option<u32>>> = lines.iter().enumerate().map(|(y, row)| {
        let mut counter = 0;
        row.char_indices().map(|(x, char)| {
            if loop_set.contains(&(x, y)) {
                match char {
                    // '|' | 'F' | '7' | 'S' => {counter += 1; None},
                    '|' | 'F' | '7' => {counter += 1; None},
                    _ => None
                }
            } else {
                Some(counter)
            }
        }).collect()
    }).collect();
    traced_grid
}

fn count_parity(traced_grid: &Vec<Vec<Option<u32>>>) -> u32 {
    let mut counter = 0;
    for row in traced_grid {
        for element in row {
            if let Some(parity) = element {
                if (parity % 2) == 1 {
                    counter += 1;
                }
            }
        }
    }
    counter
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines: Vec<String> = read_lines(filename).unwrap().map(|line| line.unwrap()).collect();

    let lines_ref: Vec<&str> = lines.iter().map(|line| line.as_str()).collect();
    let graph = parse_grid(&lines_ref);
    let start = find_starting_point(&lines_ref).unwrap();
    println!("Starting point: {:?}", start);

    let main_loop = find_loop_in_graph(&graph, start).unwrap();
    let loop_size = main_loop.len() / 2;
    println!("Loop size: {loop_size}");

    let loop_elements: HashSet<(usize, usize)> = HashSet::from_iter(main_loop);
    let enclosed_elements = count_parity(&trace_right(loop_elements, &lines_ref));
    println!("Enclosed elements: {enclosed_elements}");
}
