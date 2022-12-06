use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let (index, marker) = find_marker(&line, 4).unwrap();
        println!("The marker is {} at {}", marker, index);
        let (index, marker) = find_marker(&line, 14).unwrap();
        println!("The start of message is {} at {}", marker, index);
    }
}

fn find_marker(line: &String, length: usize) -> Option<(usize, String)> {
    let end = line.len() - length;
    for i in 0..end {
        let slice = &line[i..i+length];
        let set = &line[i..i+length].chars().collect::<HashSet<char>>();
        if set.len() == length { return Some((i+length, slice.into())) }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_marker_returns_marker() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".into();
        assert_eq!(find_marker(&input, 4), Some((7, "jpqm".into())));
    }

    #[test]
    fn find_marker_returns_marker_of_length_14() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".into();
        assert_eq!(find_marker(&input, 14), Some((19, "qmgbljsphdztnv".into())));
    }
}
