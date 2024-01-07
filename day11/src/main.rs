use std::cmp::{min, max};
use std::{path::Path, fs::File, env};
use std::io::{self, BufRead};
use std::collections::HashSet;
use itertools::Itertools;

#[derive(PartialEq, Clone, Copy)]
enum SpaceType {
    Galaxy,
    Empty,
}

struct Coord {
    x: usize,
    y: usize,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(line: &str) -> Vec<SpaceType> {
    line.chars().map(|c| match c {
        '.' => SpaceType::Empty,
        '#' => SpaceType::Galaxy,
        other => panic!("Incorrect entry: {other}"),
    }).collect()
}

fn find_empty_row_indices(grid: &Vec<Vec<SpaceType>>) -> HashSet<usize> {
    grid.iter().enumerate().filter(|(_, row)| !row.contains(&SpaceType::Galaxy)).map(|(i, _)| i).collect()
}

fn find_empty_column_indices(grid: &Vec<Vec<SpaceType>>) -> HashSet<usize> {
    let vec_size = grid[0].len();
    let mut empty_cols: HashSet<usize> = HashSet::new();

    for i in 0..vec_size {
        let non_empty = grid.iter().map(|row| row[i]).any(|e| e == SpaceType::Galaxy);
        if !non_empty {
            empty_cols.insert(i);
        }
    }
    empty_cols
}

fn compute_distance(src: &Coord, dst: &Coord, empty_rows: &HashSet<usize>, empty_cols: &HashSet<usize>) -> usize {
    let mut distance = 0;
    let (min_x, max_x) = (min(src.x, dst.x), max(src.x, dst.x));
    let (min_y, max_y) = (min(src.y, dst.y), max(src.y, dst.y));

    for x in min_x..max_x {
        if empty_cols.contains(&x) {
            // distance += 2;
            distance += 1000000;
        } else {
            distance += 1;
        }
    }

    for y in min_y..max_y {
        if empty_rows.contains(&y) {
            // distance += 2;
            distance += 1000000;
        } else {
            distance += 1;
        }
    }

    distance
}

fn find_galaxy_coordinates(grid: &Vec<Vec<SpaceType>>) -> Vec<Coord> {
    grid.iter().enumerate()
    .map(|(y, row)| row.iter().enumerate().filter(|(_, e)| **e == SpaceType::Galaxy).map(move |(x, _)| Coord {x: x, y: y}))
    .flatten().collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = read_lines(filename).unwrap();
    let parsed_grid: Vec<Vec<_>> = lines.map(|l| parse_line(&l.unwrap())).collect();

    let galaxies = find_galaxy_coordinates(&parsed_grid);
    let empty_rows = find_empty_row_indices(&parsed_grid);
    let empty_cols = find_empty_column_indices(&parsed_grid);

    let distances_sum: usize = galaxies.iter().combinations(2).map(|g| compute_distance(g[0], g[1], &empty_rows, &empty_cols)).sum();
    println!("Total distance: {distances_sum}");
}
