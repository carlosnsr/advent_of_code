use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "puzzle_input.txt";

fn sort(string: &str) -> Vec<char> {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort();
    chars
}

fn get_dupe_item(left: Vec<char>, right: Vec<char>) -> char {
    let (mut i, mut j, max) = (0, 0, left.len());
    while i < max && j < max {
        match left[i].cmp(&right[j]) {
            Ordering::Less => i += 1,
            Ordering::Greater => j += 1,
            Ordering::Equal => break,
        }
    }

    left[i]
}

fn priority(item: char) -> u32 {
    let mut score = item as u32 - 'A' as u32;
    if score < 26 { // was uppercase, bump up by 27 points
        score += 26;
    } else { // was lowercase
        score = item as u32 - 'a' as u32;
    }
    score + 1
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let middle = line.len() / 2;
        let (left, right) = (sort(&line[0..middle]), sort(&line[middle..]));
        // println!("{:?} {:?}", left, right);
        let dupe_item = get_dupe_item(left, right);
        sum += priority(dupe_item);
    }
    println!("Sum of all priorities: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_sorts_the_characters_in_a_string() {
        let expected: Vec<char> = "ABCDabcd".chars().collect();
        assert_eq!(sort("BadCAbDc"), expected);
    }

    #[test]
    fn dupe_item_returns_first_char_that_appears_in_each_string() {
        let left = vec!['a', 'c', 'e', 'f'];
        let right = vec!['b', 'd', 'e', 'g'];
        assert_eq!(get_dupe_item(left, right), 'e');
    }

    #[test]
    fn priority_returns_1_to_26_for_lowercase_char() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
    }

    #[test]
    fn priority_returns_27_to_52_for_uppercase_char() {
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }
}
