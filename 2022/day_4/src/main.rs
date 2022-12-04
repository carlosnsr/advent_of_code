use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

const FILENAME: &str = "input";

type WorkPair = RangeInclusive<usize>;

#[derive(Debug, PartialEq)]
struct Work {
    right: WorkPair,
    left: WorkPair,
}

impl Work {
    fn parse_line(line: String) -> Self {
        let mut ranges = line
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| Self::parse_pair(s))
            .collect::<Vec<WorkPair>>();
        let right = ranges.pop().unwrap();
        let left = ranges.pop().unwrap();
        Work { left: left, right: right }
    }

    fn parse_pair(input: &str) -> WorkPair{
        let numbers = input
            .split("-")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        numbers[0]..=numbers[1]
    }

    fn has_superset(&self) -> bool {
        Work::superset(&self.left, &self.right)
            || Work::superset(&self.right, &self.left)
    }

    fn superset(left: &WorkPair, right: &WorkPair) -> bool {
        left.contains(&right.start()) && left.contains(&right.end())
    }
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;
    for (index, line) in reader.lines().enumerate() {
        let work = Work::parse_line(line.unwrap());
        if work.has_superset() {
            count += 1;
        }
    }

    println!("The count is {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_parse_gives_back_work_struct() {
        let expected = Work { left: 33..=62, right: 26..=62 };
        assert_eq!(Work::parse_line("33-62,26-62".into()), expected);
    }

    #[test]
    fn work_pair_parse_gives_back_work_pair_struct() {
        let expected: WorkPair = 33..=62;
        assert_eq!(Work::parse_pair("33-62".into()), expected);
    }

    #[test]
    fn superset_returns_true_if_left_contains_right() {
        let left = 26..=62;
        let right = 33..=62;
        assert!(Work::superset(&left, &right));
        assert!(!Work::superset(&right, &left));
    }

    #[test]
    fn has_superset_returns_true_if_either_range_contains_the_other() {
        let mut input = Work { left: 33..=62, right: 26..=62 };
        assert!(input.has_superset());
        input = Work { right: 33..=62, left: 26..=62 };
        assert!(input.has_superset());
        let mut input = Work { left: 1..=9, right: 2..=10 };
        assert!(!input.has_superset());
    }
}
