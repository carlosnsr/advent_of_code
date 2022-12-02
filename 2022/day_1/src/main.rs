use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "puzzle_input.txt";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut top_four: Vec<u32> = vec![0; 4]; // 4-element zeroed-out vector
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let calories = line.parse::<u32>();
        match calories {
            Ok(calories) => sum += calories,
            Err(_) => {
                let smallest = top_four[0];
                if sum > smallest {
                    top_four[0] = sum;
                    top_four.sort_unstable();
                }
                sum = 0
            }
        }
    }

    top_four.reverse();
    top_four.pop();
    println!("The most calories carried by any one elf is {}", top_four[0]);

    let sum_three: u32 = top_four.iter().sum();
    println!("The sum of the calories carried by the top three elves is {}", sum_three);
}
