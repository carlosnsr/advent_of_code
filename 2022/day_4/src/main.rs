use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

const FILENAME: &str = "input";

#[derive(Debug, PartialEq)]
struct WorkRange {
    range: RangeInclusive<usize>,
}

impl WorkRange {
    fn parse(input: &str) -> WorkRange {
        let numbers = input
            .split("-")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        WorkRange { range: numbers[0]..=numbers[1] }
    }

    fn superset(&self, other: &Self) -> bool {
        self.range.contains(&other.range.start())
            && self.range.contains(&other.range.end())
    }
}

#[derive(Debug, PartialEq)]
struct Work {
    right: WorkRange,
    left: WorkRange,
}

impl Work {
    fn build(left: RangeInclusive<usize>, right: RangeInclusive<usize>) -> Work {
        Work {
            left: WorkRange { range: left },
            right: WorkRange { range: right },
        }
    }

    fn parse_line(line: String) -> Self {
        let mut ranges = line
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| WorkRange::parse(s))
            .collect::<Vec<WorkRange>>();
        let right = ranges.pop().unwrap();
        let left = ranges.pop().unwrap();
        Work { left: left, right: right }
    }

    fn has_superset(&self) -> bool {
        self.left.superset(&self.right) || self.right.superset(&self.left)
    }
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut count = 0;
    for (_index, line) in reader.lines().enumerate() {
        let work = Work::parse_line(line.unwrap());
        if work.has_superset() {
            count += 1;
        }
    }

    println!("The count is {}", count);
}

#[cfg(test)]
mod work_tests {
    use super::*;

    #[test]
    fn parse_returns_work() {
        let expected = Work::build(33..=62, 26..=62);
        assert_eq!(Work::parse_line("33-62,26-62".into()), expected);
    }

    #[test]
    fn has_superset_returns_true_if_either_range_contains_the_other() {
        let mut input = Work::build(33..=62, 26..=62);
        assert!(input.has_superset());
        input = Work::build(26..=62, 33..=62);
        assert!(input.has_superset());
        input = Work::build(1..=9, 2..=10);
        assert!(!input.has_superset());
    }
}

#[cfg(test)]
mod work_range_tests {
    use super::*;

    #[test]
    fn parse_returns_work_range() {
        let expected = WorkRange { range: 33..=62 };
        assert_eq!(WorkRange::parse("33-62".into()), expected);
    }

    #[test]
    fn superset_returns_true_if_left_contains_right() {
        let left = WorkRange { range: 26..=62 };
        let right = WorkRange { range: 33..=62 };
        assert!(left.superset(&right));
        assert!(!right.superset(&left));
    }
}
