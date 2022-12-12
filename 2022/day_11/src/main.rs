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
    fn examine_item(&mut self) -> (usize, usize) {
        let worry = self.items.pop_front().unwrap();
        let new_worry = self.get_new_worry(worry);
        (new_worry, self.get_next_monkey(new_worry))
    }

    fn get_new_worry(&self, worry: usize) -> usize {
        // increase worry
        let (op, amt) = &self.operation;
        let amount = match amt {
            Some(value) => *value,
            None => worry,
        };
        let mut new_worry = match op.as_ref() {
            "+" => worry + amount,
            "*" => worry * amount,
            _ => panic!(),
        };
        // decrease worry
        new_worry /= 3;

        new_worry
    }

    fn get_next_monkey(&self, worry: usize) -> usize {
        let (op, divisor, pass, fail) = &self.test;
        let result = match op.as_ref() {
            "%" => worry % divisor == 0,
            _ => panic!(),
        };
        if result { *pass } else { *fail }
    }

    fn add(&mut self, worry: usize) {
        self.items.push_back(worry);
    }
}

struct Parser {
    monkey_lines: Vec<String>,
    monkeys: Vec<Monkey>,
    inspections: Vec<usize>,
}

impl Parser {
    fn new() -> Self {
        Self { monkey_lines: Vec::new(), monkeys: Vec::new(), inspections: Vec::new() }
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
            self.inspections.push(0);
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

    fn play_rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            for i in 0..self.monkeys.len() {
                // println!("Examining monkey {}: {:?}", i, self.monkeys[i]);
                while !self.monkeys[i].items.is_empty() {
                    let (worry, next_monkey) = self.monkeys[i].examine_item();
                    self.monkeys[next_monkey].add(worry);
                    // println!("   Giving monkey {}: {:?}", next_monkey, worry);
                    self.inspections[i] += 1;
                }
            }
        }
    }

    fn monkey_business(&self) -> usize {
        let mut temp = self.inspections.clone();
        temp.sort();
        let largest = temp.pop().unwrap();
        let next_largest = temp.pop().unwrap();
        largest * next_largest
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
    // println!("{:?}", parser.monkeys);
    parser.play_rounds(20);
    println!("Part 1 monkey business: {:?}", parser.monkey_business());
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

    #[test]
    fn test_examine_item() {
        let mut monkey = Monkey {
            items: VecDeque::from([79, 98]),
            operation: ("*".into(), Some(19)),
            test: ("%".into(), 23, 2, 3)
        };
        assert_eq!(monkey.examine_item(), (500, 3));
        assert_eq!(monkey.items, VecDeque::from([98]));
    }

    #[test]
    fn test_round() {
        let input = vec![
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
            "",
            "Monkey 1:",
            "  Starting items: 54, 65, 75, 74",
            "  Operation: new = old + 6",
            "  Test: divisible by 19",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 0",
            "",
            "Monkey 2:",
            "  Starting items: 79, 60, 97",
            "  Operation: new = old * old",
            "  Test: divisible by 13",
            "    If true: throw to monkey 1",
            "    If false: throw to monkey 3",
            "",
            "Monkey 3:",
            "  Starting items: 74",
            "  Operation: new = old + 3",
            "  Test: divisible by 17",
            "    If true: throw to monkey 0",
            "    If false: throw to monkey 1",
        ];

        let mut parser = Parser::new();
        for line in input.iter() {
           parser.add(line.to_string());
        }
        // for monkey in parser.monkeys.iter() { println!("{:?}", monkey); }

        let expected_monkeys = vec![
            Monkey {
                items: VecDeque::from([79, 98]),
                operation: ("*".into(), Some(19)),
                test: ("%".into(), 23, 2, 3)
            },
            Monkey {
                items: VecDeque::from([54, 65, 75, 74]),
                operation: ("+".into(), Some(6)),
                test: ("%".into(), 19, 2, 0)
            },
            Monkey {
                items: VecDeque::from([79, 60, 97]),
                operation: ("*".into(), None),
                test: ("%".into(), 13, 1, 3)
            },
            Monkey {
                items: VecDeque::from([74]),
                operation: ("+".into(), Some(3)),
                test: ("%".into(), 17, 0, 1)
            },
        ];
        assert_eq!(parser.monkeys, expected_monkeys);

        parser.play_rounds(1);
        assert_eq!(parser.monkeys[0].items, VecDeque::from([20, 23, 27, 26]));
        assert_eq!(parser.monkeys[1].items, VecDeque::from([2080, 25, 167, 207, 401, 1046]));
        assert!(parser.monkeys[2].items.is_empty());
        assert!(parser.monkeys[3].items.is_empty());

        parser.play_rounds(19);
        assert_eq!(parser.monkeys[0].items, VecDeque::from([10, 12, 14, 26, 34]));
        assert_eq!(parser.monkeys[1].items, VecDeque::from([245, 93, 53, 199, 115]));
        assert!(parser.monkeys[2].items.is_empty());
        assert!(parser.monkeys[3].items.is_empty());
        assert_eq!(parser.inspections, vec![101, 95, 7, 105]);
        assert_eq!(parser.monkey_business(), 101 * 105);
    }
}
