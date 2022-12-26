use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use template::bfsearch::{Grid, Node, bfsearch};
use template::hill_climb::climb_to_top;

const FILENAME: &str = "input";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut grid: Grid<Node> = Grid::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        grid.push(&line);
    }

    println!("The grid is {:?}", grid);
    // if let Some(path) = bfsearch(&mut grid) {
    if let Some(path) = climb_to_top(&mut grid) {
        println!("The shortestt number of steps is {}", path.len() - 1);
    } else {
        println!("No path was found");
    }
}
