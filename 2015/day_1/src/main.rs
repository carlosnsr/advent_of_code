use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input";

fn decode(c: char) -> isize {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0
    }
}

fn find_floor(input: &String) -> isize {
    input
        .chars()
        .fold(0, |acc, x| acc + decode(x))
}

fn enters_basement(input: &String) -> Option<usize> {
    let chars: Vec<char> = input.chars().collect();
    let mut sum = 0;
    for i in 0..chars.len() {
        sum += decode(chars[i]);
        if sum == -1 {
            return Some(i)
        }
    }
    None
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut basement_when = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        sum += find_floor(&line);
        basement_when = enters_basement(&line).unwrap() + 1; // not 0-indexed
    }

    println!("The sum is {}", sum);
    println!("Enters basement {}", basement_when);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(find_floor(&"(())".into()), 0);
        assert_eq!(find_floor(&"()()".into()), 0);
    }
}
