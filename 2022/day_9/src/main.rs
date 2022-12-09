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
            x: if self.x.abs() > 1 { (self.x/2)*(self.x.abs() - 1) } else { self.x },
            y: if self.y.abs() > 1 { (self.y/2)*(self.y.abs() - 1) } else { self.y },
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

    fn travel(&mut self, direction: &str, distance: usize) {
        let tail_i = self.knots.len() - 1;
        for _ in 0..distance {
            for i in 0..tail_i {
                let mut step = Point::new(0, 0);
                match direction {
                    "R" => step.x += 1,
                    "L" => step.x -= 1,
                    "U" => step.y += 1,
                    "D" => step.y -= 1,
                    _ => panic!(),
                }
                self.knots[i].add(&step);

                let distance = self.knots[i+1].distance(&self.knots[i]);
                let change = distance.ord();
                if !(distance == change) {
                    self.knots[i+1].add(&change);
                    if i+1 == tail_i {
                        self.visited.insert(self.knots[tail_i].clone());
                    }
                }
            }
        }
    }
}

fn parse_instruction(line: &String) -> (&str, usize) {
    let mut tokens: Vec<&str> = line.split(" ").collect();
    let distance = tokens.pop().unwrap().parse::<usize>().unwrap();
    let direction = tokens.pop().unwrap();
    (direction, distance)
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut rope = Rope::new(2);
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let (distance, direction) = parse_instruction(&line);
        rope.travel(distance, direction);
    }
    println!("Tail visited {} positions", rope.visited.len());

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_knot_rope_travelling() {
        let mut rope = Rope::new(2);

        rope.travel("R", 4);
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

        rope.travel("U", 4);
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

        rope.travel("L", 3);
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

        rope.travel("D", 1);
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
