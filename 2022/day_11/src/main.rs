use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "input";

type OpsType = (String, Option<usize>);
type TestType = (String, usize, usize, usize);

#[derive(Debug, PartialEq)]
struct Monkey {
    items: VecDeque<usize>,
    operation: OpsType,
    test: TestType,
}

impl Monkey {
}

struct Parser {
    monkey_lines: Vec<String>,
    monkeys: Vec<Monkey>,
}

impl Parser {
    fn new() -> Self {
        Self { monkey_lines: Vec::new(), monkeys: Vec::new() }
    }

    fn add(&mut self, line: String) {
        if line.is_empty() {
            return;
        }

        self.monkey_lines.push(line);
        if self.monkey_lines.len() == 6 {
            let monkey = Parser::make_monkey(&self.monkey_lines);
            self.monkey_lines.clear();
            self.monkeys.push(monkey);
        }
    }

    fn make_monkey(lines: &Vec<String>) -> Monkey {
        // println!("{:?}", lines);
        Monkey {
            items: Parser::parse_items_line(&lines[1]),
            operation: Parser::parse_ops_line(&lines[2]),
            test: Parser::parse_test_lines(&lines),
        }
    }

    fn parse_items_line(line: &String) -> VecDeque<usize> {
        let colon_i = line.find(":").unwrap() + 2;
        let items: VecDeque<usize> = line[colon_i..]
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
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
            "*" | "+" => (caps["op"].into(), amt),
            _ => panic!(),
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

        ("%".into(), amt, pass_monkey, fail_monkey)
    }
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    // read in input and make monkeys
    let mut parser = Parser::new();
    for line in reader.lines() {
        let line = line.unwrap();
        parser.add(line);
    }
    println!("{:?}", parser.monkeys);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ops_line() {
        let input: String = "    Operation: new = old * 19".into();
        let expected = ("*".into(), Some(19));
        assert_eq!(Parser::parse_ops_line(&input), expected);

        let input: String = "    Operation: new = old + 3".into();
        let expected = ("+".into(), Some(3));
        assert_eq!(Parser::parse_ops_line(&input), expected);

        let input: String = "    Operation: new = old * old".into();
        let expected = ("*".into(), None);
        assert_eq!(Parser::parse_ops_line(&input), expected);
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
            items: VecDeque::from([79, 98]),
            operation: ("*".into(), Some(19)),
            test: ("%".into(), 23, 2, 3),
        };
        let monkey = Parser::make_monkey(&monkey_lines);
        assert_eq!(monkey, expected);
    }
}
