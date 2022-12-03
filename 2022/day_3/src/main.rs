use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "puzzle_input.txt";

fn find_dupe_items(left: &String, right: &String) -> String {
    let left_set: HashSet<char> = left.chars().collect();
    let right_set: HashSet<char> = right.chars().collect();
    left_set.intersection(&right_set)
        .into_iter()
        .map(|x| *x)
        .collect::<String>()
}

fn get_badge(trio: &Vec<String>) -> String {
    let intersection = find_dupe_items(&trio[0], &trio[1]);
    let badges = find_dupe_items(&intersection, &trio[2]);
    if badges.len() == 1 {
        badges
    } else {
        panic!("Whoops: trio:{:?}  dupes:{:?}", trio, badges)
    }
}

fn priority(items: &String) -> u32 {
    let item = items.chars().next().unwrap();
    let score = item as u32 - 'A' as u32;
    if score < 26 { // was uppercase, bump up by 27 points
        score + 27
    } else { // was lowercase
        item as u32 - 'a' as u32 + 1
    }
}

fn get_rucksack_item_priority(line: &str) -> u32 {
    let middle = line.len() / 2;
    let (left, right) = (&line[0..middle].into(), &line[middle..].into());
    let dupe = find_dupe_items(left, right);
    if dupe.len() == 1 {
        priority(&dupe)
    } else {
        panic!("Whoops: left:{:?} right:{:?} dupe:{:?}", left, right, dupe)
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
            trio_sum += priority(&badge);
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
    fn find_dupe_items_finds_all_duplicate_items() {
        let left = "Aacef".into();
        let right = "bdefg".into();
        let actual = find_dupe_items(&left, &right);
        assert!(["ef", "fe"].iter().any(|s| s == &actual.as_str()));
    }

    #[test]
    fn priority_returns_1_to_26_for_lowercase_char() {
        assert_eq!(priority(&"a".into()), 1);
        assert_eq!(priority(&"z".into()), 26);
    }

    #[test]
    fn priority_returns_27_to_52_for_uppercase_char() {
        assert_eq!(priority(&"A".into()), 27);
        assert_eq!(priority(&"Z".into()), 52);
    }

    #[test]
    fn get_badge_returns_the_char_common_to_the_trio() {
        let mut trio = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".into(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".into(),
            "PmmdzqPrVvPwwTWBwg".into(),
        ];
        assert_eq!(get_badge(&trio), "r");
        trio = vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".into(),
            "ttgJtRGJQctTZtZT".into(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".into(),
        ];
        assert_eq!(get_badge(&trio), "Z");
    }
}
