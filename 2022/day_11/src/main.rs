use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "input";

#[derive(Debug, PartialEq)]
struct Item {
    worry_level: usize,
}

impl Item {
    fn new(worry_level: usize) -> Self {
        Self { worry_level }
    }
}

// type OpsFn = fn(usize) -> usize;
type OpsFn = fn(usize) -> usize;
type TestFn = fn(usize) -> usize;
type OpsType = Option<(String, Option<usize>)>;
type TestType = Option<(String, usize, usize, usize)>;


#[derive(Debug, PartialEq)]
struct Monkey {
    items: VecDeque<Item>,
    operation: OpsType,
    test: TestType,
}

impl Monkey {
    fn make(lines: &Vec<String>) -> Self {
        Monkey {
            items: Monkey::parse_items_line(&lines[1]),
            operation: Monkey::parse_ops_line(&lines[2]),
            test: Monkey::parse_test_lines(&lines),
        }
    }

    fn parse_items_line(line: &String) -> VecDeque<Item> {
        let colon_i = line.find(":").unwrap() + 2;
        let items: VecDeque<Item> = line[colon_i..]
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .map(|w| Item::new(w))
            .collect();
        items
    }

    fn parse_ops_line(line: &String) -> OpsType {
        lazy_static! {
            static ref OPS_RE: Regex = Regex::new(
                r"^\s+Operation: new = old (?P<op>[*+]) (?P<amt>\w+)$"
            ).unwrap();
        }
        assert!(OPS_RE.is_match(line));
        let caps = OPS_RE.captures(line).unwrap();
        // println!("{:?} {:?}", &caps["op"], &caps["amt"]);
        let amt = match &caps["amt"] {
            "old" => None,
            _ => Some(caps["amt"].parse().unwrap()),
        };
        match &caps["op"] {
            "*" | "+" => Some((caps["op"].into(), amt)),
            _ => None,
        }
    }

    fn parse_test_lines(lines: &Vec<String>) -> TestType {
        lazy_static! {
            static ref TEST_RE: Regex = Regex::new(
                r"^\s+Test: divisible by (?P<amt>\d+)$"
            ).unwrap();
            static ref BRANCH_RE: Regex = Regex::new(
                r"^\s+If (true|false): throw to monkey (?P<monkey>\d+)$"
            ).unwrap();
        }
        let test_line = &lines[3];
        assert!(TEST_RE.is_match(test_line));
        let amt: usize = TEST_RE.captures(test_line).unwrap()["amt"].parse().unwrap();

        let true_line = &lines[4];
        assert!(BRANCH_RE.is_match(true_line));
        let pass_monkey: usize = BRANCH_RE.captures(true_line).unwrap()["monkey"].parse().unwrap();

        let false_line = &lines[5];
        assert!(BRANCH_RE.is_match(false_line));
        let fail_monkey: usize = BRANCH_RE.captures(false_line).unwrap()["monkey"].parse().unwrap();

        Some(("%".into(), amt, pass_monkey, fail_monkey))
    }
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // read in input and make monkeys
    let mut sum = 0;
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_lines: Vec<String> = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            println!("{:?}", monkey_lines);
            monkeys.push(Monkey::make(&monkey_lines));
            monkey_lines.clear();
        } else {
            monkey_lines.push(line);
        }
    }
    monkeys.push(Monkey::make(&monkey_lines));
    println!("{:?}", monkeys);

    println!("The sum is {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ops_line() {
        let input: String = "    Operation: new = old * 19".into();
        let expected = Some(("*".into(), Some(19)));
        assert_eq!(Monkey::parse_ops_line(&input), expected);

        let input: String = "    Operation: new = old + 3".into();
        let expected = Some(("+".into(), Some(3)));
        assert_eq!(Monkey::parse_ops_line(&input), expected);

        let input: String = "    Operation: new = old * old".into();
        let expected = Some(("*".into(), None));
        assert_eq!(Monkey::parse_ops_line(&input), expected);
    }

    #[test]
    fn test_make_monkey() {
        let monkey_lines: Vec<String> = Vec::from([
            "Monkey 0:".into(),
            "    Starting items: 79, 98".into(),
            "    Operation: new = old * 19".into(),
            "    Test: divisible by 23".into(),
            "        If true: throw to monkey 2".into(),
            "        If false: throw to monkey 3".into(),
        ]);
        let expected = Monkey {
            items: VecDeque::from([Item::new(79), Item::new(98)]),
            operation: Some(("*".into(), Some(19))),
            test: Some(("%".into(), 23, 2, 3)),
        };
        let monkey = Monkey::make(&monkey_lines);
        assert_eq!(monkey, expected);
    }
}
