use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "puzzle_input.txt";

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
struct Round {
    opponent: Hand,
    mine: Hand,
}

impl Round {
    fn build(opponent: &str, mine: &str) -> Round {
        Round {
            opponent: codex(opponent),
            mine: codex(mine),
        }
    }

    fn score(&self) -> u32 {
        let hand_score = match self.mine {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };
        let outcome_score = match outcome(&self.opponent, &self.mine) {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        };

        hand_score + outcome_score
    }
}

fn codex(hand: &str) -> Hand {
    match hand {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissors,
        _ => panic!("Unknown value {}", hand),
    }
}

fn outcome(opponent: &Hand, mine: &Hand) -> Outcome {
    if opponent == mine {
        Outcome::Draw
    } else {
        match opponent {
            Hand::Rock => match mine {
                Hand::Scissors => Outcome::Loss,
                _ => Outcome::Win,
            }
            Hand::Paper => match mine {
                Hand::Rock => Outcome::Loss,
                _ => Outcome::Win,
            }
            Hand::Scissors => match mine {
                Hand::Paper => Outcome::Loss,
                _ => Outcome::Win,
            }
        }
    }
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let plays: Vec<&str> = line.split(" ").collect();
        let round = Round::build(plays[0], plays[1]);
        score += round.score();
    }
    println!("Total score: {:?}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_converts_chars_to_hands() {
        let actual = Round::build("A", "X");
        let expected = Round { opponent: Hand::Rock, mine: Hand::Rock };
        assert_eq!(actual, expected);
    }

    #[test]
    fn codex_converts_chars_to_hands() {
        assert_eq!(codex("A"), Hand::Rock);
        assert_eq!(codex("B"), Hand::Paper);
        assert_eq!(codex("C"), Hand::Scissors);
        assert_eq!(codex("X"), Hand::Rock);
        assert_eq!(codex("Y"), Hand::Paper);
        assert_eq!(codex("Z"), Hand::Scissors);
    }

    #[test]
    fn outcome_correcly_calls_draws() {
        assert_eq!(outcome(&Hand::Rock, &Hand::Rock), Outcome::Draw);
        assert_eq!(outcome(&Hand::Paper, &Hand::Paper), Outcome::Draw);
        assert_eq!(outcome(&Hand::Scissors, &Hand::Scissors), Outcome::Draw);
    }

    #[test]
    fn outcome_correcly_calls_losses() {
        assert_eq!(outcome(&Hand::Rock, &Hand::Scissors), Outcome::Loss);
        assert_eq!(outcome(&Hand::Paper, &Hand::Rock), Outcome::Loss);
        assert_eq!(outcome(&Hand::Scissors, &Hand::Paper), Outcome::Loss);
    }
}
