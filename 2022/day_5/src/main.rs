use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input";

const STACK_NUMBERS_LINE: usize = 8;

type Stack = Vec<String>;
type Stacks = Vec<Stack>;

fn parse(crate_lines: &mut Vec<String>) -> Stacks {
    let max_stacks = (crate_lines[0].len() + 1)/4;
    let mut stacks: Stacks = vec![Vec::new(); max_stacks];
    for _i in 0..crate_lines.len() {
        let line = crate_lines.pop().unwrap();
        for stack in 0..max_stacks {
            let pos = 1 + (stack * 4);
            let content  = String::from(&line[pos..pos+1]);
            if !(content  == " ") {
                stacks[stack].push(content);
            }
        }
    }

    stacks
}

fn perform_stack_move(line: String, stacks: &mut Stacks) {
    let instruction = Instruction::parse(line);
    for _i in 0..instruction.amount {
        let crate_ = stacks[instruction.from].pop().unwrap();
        stacks[instruction.to].push(crate_);
    }
}

#[derive(Debug, PartialEq)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn parse(line: String) -> Instruction {
        let fields = line.split(" ").collect::<Vec<&str>>();
        Instruction {
            amount: fields[1].parse().unwrap(),
            from: fields[3].parse::<usize>().unwrap() - 1,
            to: fields[5].parse::<usize>().unwrap() - 1,
        }
    }
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut crate_lines = Vec::with_capacity(STACK_NUMBERS_LINE);
    let mut stacks = Vec::new();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        // read in stacks (first 8 lines)
        if index < STACK_NUMBERS_LINE {
            crate_lines.push(line);
        } else if index == STACK_NUMBERS_LINE {
            stacks = parse(&mut crate_lines);
            // println!("{:?}", stacks);
        // read and perform instructions
        } else if index >= 10 {
            perform_stack_move(line, &mut stacks);
        }
    }

    let top_row = stacks
        .iter()
        .map(|stack| stack[stack.len() - 1].clone())
        .collect::<Vec<String>>()
        .join("");

    println!("The top row is {}", top_row);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_crates_returns_crates() {
        let mut crate_lines = vec![
            "[Q]     [W]".into(),
            "[N] [B] [Q]".into(),
            "[H] [W] [S]".into(),
        ];
        let expected: Stacks = vec![
            vec!["H".into(), "N".into(), "Q".into()],
            vec!["W".into(), "B".into()],
            vec!["S".into(), "Q".into(), "W".into()],
        ];
        assert_eq!(parse(&mut crate_lines), expected);
    }

    #[test]
    fn parse_instructions_returns_number_crate_target() {
        let input = "move 10 from 2 to 7".into();
        let expected = Instruction {
            amount: 10,
            from: 1,
            to: 6,
        };
        assert_eq!(Instruction::parse(input), expected);
    }
}
