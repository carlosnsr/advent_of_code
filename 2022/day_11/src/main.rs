use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

#[derive(Debug, PartialEq)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Option<fn(usize) -> usize>,
    test: Option<fn(usize) -> usize>,
}

impl Monkey {
    fn make(lines: &VecDeque<String>) -> Self {
        // let mut iter = lines.iter();
        // println!("{:?}", line);

        Monkey {
            items: Monkey::parse_items(&lines[1]),
            operation: None,
            test: None,
        }
    }

    fn parse_items(line: &String) -> VecDeque<Item> {
        let colon_i = line.find(":").unwrap() + 2;
        let items: VecDeque<Item> = line[colon_i..]
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .map(|w| Item::new(w))
            .collect();
        items
    }
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut monkey_lines: VecDeque<String> = VecDeque::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        monkey_lines.push_back(line);
    }

    println!("{:?}", monkey_lines);
    println!("The sum is {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let monkey_lines: VecDeque<String> = VecDeque::from([
            "Monkey 0:".into(),
            "    Starting items: 79, 98".into(),
            "    Operation: new = old * 19".into(),
            "    Test: divisible by 23".into(),
            "        If true: throw to monkey 2".into(),
            "        If false: throw to monkey 3".into(),
        ]);
        let expected = Monkey {
            items: VecDeque::from([Item::new(79), Item::new(98)]),
            operation: Some(|x| x * 19),
            test: Some(|x| if x % 23 == 0 { 2 } else { 3 }),
        };
        let monkey = Monkey::make(&monkey_lines);
        assert_eq!(monkey, expected);
    }
}
