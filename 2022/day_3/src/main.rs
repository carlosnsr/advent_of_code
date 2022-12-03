use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "puzzle_input.txt";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
