use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "puzzle_input.txt";

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn codex(outcome: &str) -> Outcome {
        match outcome {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Unknown value {}", outcome),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn codex(hand: &str) -> Hand {
        match hand {
            "A" => Hand::Rock,
            "B" => Hand::Paper,
            "C" => Hand::Scissors,
            _ => panic!("Unknown value {}", hand),
        }
    }

    fn beats(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Round {
    opponent: Hand,
    outcome: Outcome,
}

impl Round {
    fn build(opponent: &str, outcome: &str) -> Round {
        Round {
            opponent: Hand::codex(opponent),
            outcome: Outcome::codex(outcome),
        }
    }

    fn make_my_hand(&self) -> Hand {
        match self.outcome {
            Outcome::Draw => self.opponent.clone(),
            Outcome::Loss => self.opponent.beats(),
            Outcome::Win => self.opponent.beats().beats(),
        }
    }

    fn score(&self) -> u32 {
        let mine = self.make_my_hand();
        let hand_score = match mine {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        };

        let outcome_score = match self.outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        };

        hand_score + outcome_score
    }
}

fn outcome(opponent: &Hand, mine: &Hand) -> Outcome {
    if opponent == mine {
        Outcome::Draw
    } else if &opponent.beats() == mine {
        Outcome::Loss
    } else {
        Outcome::Win
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
    fn round_converts_chars_to_hands_and_outcome() {
        let actual = Round::build("A", "X");
        let expected = Round { opponent: Hand::Rock, outcome: Outcome::Loss };
        assert_eq!(actual, expected);
    }

    #[test]
    fn round_calls_correct_hand_for_win() {
        let round = Round { opponent: Hand::Rock, outcome: Outcome::Win };
        assert_eq!(round.make_my_hand(), Hand::Paper);
    }

    #[test]
    fn round_calls_correct_hand_for_draw() {
        let round = Round { opponent: Hand::Rock, outcome: Outcome::Draw };
        assert_eq!(round.make_my_hand(), Hand::Rock);
    }

    #[test]
    fn round_calls_correct_hand_for_loss() {
        let round = Round { opponent: Hand::Rock, outcome: Outcome::Loss };
        assert_eq!(round.make_my_hand(), Hand::Scissors);
    }

    #[test]
    fn hands_codex_converts_chars_to_hands() {
        assert_eq!(Hand::codex("A"), Hand::Rock);
        assert_eq!(Hand::codex("B"), Hand::Paper);
        assert_eq!(Hand::codex("C"), Hand::Scissors);
    }

    fn hand_knows_which_hand_it_beats() {
        assert_eq!(Hand::Rock.beats(), Hand::Scissors);
    }

    #[test]
    fn outcomes_codex_converts_chars_to_hands() {
        assert_eq!(Outcome::codex("X"), Outcome::Loss);
        assert_eq!(Outcome::codex("Y"), Outcome::Draw);
        assert_eq!(Outcome::codex("Z"), Outcome::Win);
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
