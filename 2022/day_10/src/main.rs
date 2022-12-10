use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input";

enum Command {
    NoOp,
    AddX(isize),
}

#[derive(Debug)]
struct Register {
    value: isize,
    cycles: usize,
    sum: isize,
}

impl Register {
    fn new() -> Self {
        Self { value: 1, cycles: 0, sum: 0 }
    }

    fn noop(&mut self) {
        self.tick();
    }

    fn addx(&mut self, value: isize) {
        self.tick();
        self.tick();
        self.value += value;
    }

    fn tick(&mut self) {
        self.cycles += 1;
        if self.is_signal() {
            self.sum += self.signal_strength();
            println!("{:?} {:?}", self, self.signal_strength());
        }
    }

    fn is_signal(&self) -> bool {
        match self.cycles {
            20 => true,
            60 => true,
            100 => true,
            140 => true,
            180 => true,
            220 => true,
            _ => false,
        }
    }

    fn signal_strength(&self) -> isize {
         self.value * (self.cycles as isize)
    }

    fn parse(line: &String) -> Command {
        let tokens: Vec<&str> = line.split(" ").collect();
        match tokens[0] {
            "noop" => Command::NoOp,
            "addx" => Command::AddX(tokens[1].parse::<isize>().unwrap()),
            _ => panic!(),
        }
    }
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut register = Register::new();
    for (_index, line) in reader.lines().enumerate() {
        match Register::parse(&line.unwrap()) {
            Command::NoOp => register.noop(),
            Command::AddX(value) => register.addx(value),
        }
    }
    println!("{}", register.sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<&'static str> {
        vec![
            "addx 15",
            "addx -11",
            "addx 6",
            "addx -3",
            "addx 5",
            "addx -1",
            "addx -8",
            "addx 13",
            "addx 4",
            "noop",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx 5",
            "addx -1",
            "addx -35",
            "addx 1",
            "addx 24",
            "addx -19",
            "addx 1",
            "addx 16",
            "addx -11",
            "noop",
            "noop",
            "addx 21",
            "addx -15",
            "noop",
            "noop",
            "addx -3",
            "addx 9",
            "addx 1",
            "addx -3",
            "addx 8",
            "addx 1",
            "addx 5",
            "noop",
            "noop",
            "noop",
            "noop",
            "noop",
            "addx -36",
            "noop",
            "addx 1",
            "addx 7",
            "noop",
            "noop",
            "noop",
            "addx 2",
            "addx 6",
            "noop",
            "noop",
            "noop",
            "noop",
            "noop",
            "addx 1",
            "noop",
            "noop",
            "addx 7",
            "addx 1",
            "noop",
            "addx -13",
            "addx 13",
            "addx 7",
            "noop",
            "addx 1",
            "addx -33",
            "noop",
            "noop",
            "noop",
            "addx 2",
            "noop",
            "noop",
            "noop",
            "addx 8",
            "noop",
            "addx -1",
            "addx 2",
            "addx 1",
            "noop",
            "addx 17",
            "addx -9",
            "addx 1",
            "addx 1",
            "addx -3",
            "addx 11",
            "noop",
            "noop",
            "addx 1",
            "noop",
            "addx 1",
            "noop",
            "noop",
            "addx -13",
            "addx -19",
            "addx 1",
            "addx 3",
            "addx 26",
            "addx -30",
            "addx 12",
            "addx -1",
            "addx 3",
            "addx 1",
            "noop",
            "noop",
            "noop",
            "addx -9",
            "addx 18",
            "addx 1",
            "addx 2",
            "noop",
            "noop",
            "addx 9",
            "noop",
            "noop",
            "noop",
            "addx -1",
            "addx 2",
            "addx -37",
            "addx 1",
            "addx 3",
            "noop",
            "addx 15",
            "addx -21",
            "addx 22",
            "addx -6",
            "addx 1",
            "noop",
            "addx 2",
            "addx 1",
            "noop",
            "addx -10",
            "noop",
            "noop",
            "addx 20",
            "addx 1",
            "addx 2",
            "addx 2",
            "addx -6",
            "addx -11",
            "noop",
            "noop",
            "noop",
        ]
    }

    #[test]
    fn test() {
        let mut register = Register::new();
        let mut last_sum = register.sum;
        for (index, line) in input().iter().enumerate() {
            let line: String = (*line).into();
            match Register::parse(&line) {
                Command::NoOp => register.noop(),
                Command::AddX(value) => register.addx(value),
            }
        }
        assert_eq!(register.sum, 13140);
    }
}
