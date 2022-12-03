use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "puzzle_input.txt";

fn sort(string: &str) -> Vec<char> {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort();
    chars
}

fn find_dupe_items(left: &Vec<char>, right: &Vec<char>) -> Vec<char> {
    let left_set: HashSet<&char> = left.into_iter().collect();
    let right_set: HashSet<&char> = right.into_iter().collect();
    let answer: Vec<char> = left_set
        .intersection(&right_set)
        .into_iter()
        .map(|x| **x)
        .collect();
    answer
}

fn get_badge(trio: &Vec<String>) -> char {
    let trio: Vec<Vec<char>> = trio
        .iter()
        .map(|line| sort(line))
        .collect();

    let intersection = find_dupe_items(&trio[0], &trio[1]);
    let badges = find_dupe_items(&intersection, &trio[2]);
    if badges.len() == 1 {
        badges[0]
    } else {
        panic!("Whoops: trio:{:?}  dupes:{:?}", trio, badges)
    }

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

fn get_rucksack_item_priority(line: &str) -> u32 {
    let middle = line.len() / 2;
    let (left, right) = (sort(&line[0..middle]), sort(&line[middle..]));
    let dupes = find_dupe_items(&left, &right);
    if dupes.len() == 1 {
        priority(dupes[0])
    } else {
        panic!("Whoops: left:{:?} right:{:?} dupes:{:?}", left, right, dupes)
    }
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut trio: Vec<String> = vec![];
    let mut trio_sum = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        sum += get_rucksack_item_priority(&line);
        trio.push(line);
        if trio.len() == 3 {
            let badge = get_badge(&trio);
            trio_sum += priority(badge);
            trio.clear();
        }
    }
    println!("Sum of all priorities: {}", sum);
    println!("Sum of all trio priorities: {}", trio_sum);
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
    fn find_dupe_items_finds_all_duplicate_items() {
        let left = vec!['A', 'a', 'c', 'e', 'f'];
        let right = vec!['b', 'd', 'e', 'f', 'g'];
        assert_eq!(find_dupe_items(&left, &right), vec!['e', 'f']);
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

    #[test]
    fn get_badge_returns_the_char_common_to_the_trio() {
        let mut trio = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".into(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".into(),
            "PmmdzqPrVvPwwTWBwg".into(),
        ];
        assert_eq!(get_badge(&trio), 'r');
        trio = vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".into(),
            "ttgJtRGJQctTZtZT".into(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".into(),
        ];
        assert_eq!(get_badge(&trio), 'Z');
    }
}
