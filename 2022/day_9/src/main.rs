use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
        }
    }

    fn distance(&self, target: &Point) -> Self {
        Self {
            x: target.x - self.x,
            y: target.y - self.y,
        }
    }

    fn ord(&self) -> Self {
        Self {
            x: Point::ord_value(self.x),
            y: Point::ord_value(self.y),
        }
    }

    fn ord_value(v: isize) -> isize {
        if v.abs() > 1 {
            (v/v.abs())*(v.abs() - 1)
        } else {
            v
        }
    }

    fn add(&mut self, target: &Point) {
        self.x += target.x;
        self.y += target.y;
    }
}

#[derive(Debug, PartialEq)]
struct Rope {
    knots: Vec<Point>,
    visited: HashSet<Point>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Self {
            knots: vec![Point::new(0, 0); length],
            visited: HashSet::from([Point::new(0, 0)]),
        }
    }

    fn make_step(direction: &str) -> Point {
        let mut step = Point::new(0, 0);
        match direction {
            "R" => step.x += 1,
            "L" => step.x -= 1,
            "U" => step.y += 1,
            "D" => step.y -= 1,
            _ => panic!(),
        }

        step
    }

    fn travel(&mut self, instruction: &Instruction) {
        let mut distance;
        let mut change;
        let tail_i = self.knots.len() - 1;
        let step = Rope::make_step(instruction.direction);
        for _ in 0..instruction.distance {
            // update head
            self.knots[0].add(&step);

            // update subsequent knots
            for i in 1..self.knots.len() {
                distance = self.knots[i].distance(&self.knots[i-1]);
                change = distance.ord();
                if distance == change {
                    break;
                } else {
                    self.knots[i].add(&change);
                    if i == tail_i { // i.e. is tail. register its new position
                        self.visited.insert(self.knots[tail_i].clone());
                    }
                }
            }
        }
    }
}

struct Instruction<'a> {
    direction: &'a str,
    distance: usize,
}

impl<'a> Instruction<'a> {
    fn new(direction: &'a str, distance: usize) -> Self {
        Self {
            direction,
            distance,
        }
    }

    fn parse(line: &'a String) -> Self {
        let tokens: Vec<&str> = line.split(" ").collect();
        Self::new(tokens[0], tokens[1].parse::<usize>().unwrap())
    }
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut rope = Rope::new(2);
    let mut rope2 = Rope::new(10);
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let instruction = Instruction::parse(&line);
        rope.travel(&instruction);
        rope2.travel(&instruction);
    }
    println!("Part 1: Tail visited {} positions", rope.visited.len());
    println!("Part 2: Tail visited {} positions", rope2.visited.len());

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_knot_rope_travelling() {
        let mut rope = Rope::new(2);

        rope.travel(&Instruction::new("R", 4));
        let mut expected = Rope {
            knots: vec![Point::new(4, 0), Point::new(3, 0)],
            visited: HashSet::from([
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
            ]),
        };
        assert_eq!(rope, expected);

        rope.travel(&Instruction::new("U", 4));
        expected = Rope {
            knots: vec![Point::new(4, 4), Point::new(4, 3)],
            visited: HashSet::from([
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(4, 1),
                Point::new(4, 2),
                Point::new(4, 3),
            ]),
        };
        assert_eq!(rope, expected);

        rope.travel(&Instruction::new("L", 3));
        expected = Rope {
            knots: vec![Point::new(1, 4), Point::new(2, 4)],
            visited: HashSet::from([
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(4, 1),
                Point::new(4, 2),
                Point::new(4, 3),
                Point::new(3, 4),
                Point::new(2, 4),
            ]),
        };
        assert_eq!(rope, expected);

        rope.travel(&Instruction::new("D", 1));
        expected = Rope {
            knots: vec![Point::new(1, 3), Point::new(2, 4)],
            visited: HashSet::from([
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(4, 1),
                Point::new(4, 2),
                Point::new(4, 3),
                Point::new(3, 4),
                Point::new(2, 4),
            ]),
        };
        assert_eq!(rope, expected);
    }
}
