use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "puzzle_input.txt";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut largest = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let calories = line.parse::<u32>();
        match calories {
            Ok(calories) => sum += calories,
            Err(_) => {
                if sum > largest {
                    largest = sum;
                }
                sum = 0
            }
        }
    }
    println!("The most calories carried by any one elf is {}", largest);
}
