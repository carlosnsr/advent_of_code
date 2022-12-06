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
        let set = slice.chars().collect::<HashSet<char>>();
        if set.len() == length { return Some((i+length, slice.into())) }
    }

    None
}

// alternative implementation using a VecQueue
// fn find_marker(line: &String, length: usize) -> Option<(usize, String)> {
//     let mut queue: VecDeque<char> = VecDeque::with_capacity(length);
//     let mut set: HashSet<char> = HashSet::with_capacity(length);
//
//     for (i, letter) in line.chars().enumerate() {
//         queue.push_back(letter);
//
//         if queue.len() == length {
//             queue.iter().for_each(|x| { set.insert(*x); });
//             if set.len() == length {
//                 return Some((i+1, String::from(&line[i+1-length..i+1])));
//             }
//             queue.pop_front().unwrap();
//             set.clear();
//         }
//     }
//
//     None
// }

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
