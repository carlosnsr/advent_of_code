use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input";

type WorkPair = (usize, usize);

#[derive(Debug, PartialEq)]
struct Work {
    left: WorkPair,
    right: WorkPair,
}

impl Work {
    fn parse_line(line: String) -> Self {
        let pairs = line
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| Self::parse_pair(s))
            .collect::<Vec<WorkPair>>();
        Work { left: pairs[0], right: pairs[1] }
    }

    fn parse_pair(input: &str) -> WorkPair{
        let numbers = input
            .split("-")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        (numbers[0], numbers[1])
    }
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    for (index, line) in reader.lines().enumerate() {
        let work = Work::parse_line(line.unwrap());
        println!("{:?}", work);
        if index == 10 {
          break;
        }
    }

    println!("The sum is {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_parse_gives_back_work_struct() {
        let expected = Work { left: (33, 62), right: (26, 62) };
        assert_eq!(Work::parse_line("33-62,26-62".into()), expected);
    }

    #[test]
    fn work_pair_parse_gives_back_work_pair_struct() {
        let expected: WorkPair = (33, 62);
        assert_eq!(Work::parse_pair("33-62".into()), expected);
    }
}
