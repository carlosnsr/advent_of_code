use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let (index, marker) = find_marker(&line).unwrap();
        println!("The marker is {} at {}", marker, index);
    }
}

fn find_marker(line: &String) -> Option<(usize, String)> {
    for i in 0..(line.len()-4) {
        let slice = &line[i..i+4];
        let set = &line[i..i+4].chars().collect::<HashSet<char>>();
        if set.len() == 4 { return Some((i+3+1, slice.into())) }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_marker_returns_marker() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".into();
        assert_eq!(find_marker(&input), Some((7, "jpqm".into())));
    }
}
