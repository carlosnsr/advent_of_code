use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }

    println!("The sum is {}", sum);
}
