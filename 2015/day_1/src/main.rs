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

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    for (_index, line) in reader.lines().enumerate() {
        sum += find_floor(&line.unwrap());
    }

    println!("The sum is {}", sum);
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
