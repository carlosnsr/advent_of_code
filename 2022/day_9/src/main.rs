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

    fn distance(&self, target: &Point) -> usize {
        let distance = [
            target.x - self.x,
            target.y - self.y,
        ]
            .iter()
            .map(|v| v.abs())
            .max()
            .unwrap();

        distance as usize
    }
}

#[derive(Debug, PartialEq)]
struct Rope {
    head: Point,
    tail: Point,
    visited: HashSet<Point>,
}

impl Rope {
    fn new() -> Self {
        Self {
            head: Point::new(0, 0),
            tail: Point::new(0, 0),
            visited: HashSet::from([Point::new(0, 0)]),
        }
    }

    fn travel(&mut self, direction: &str, distance: usize) {
        for _i in 0..distance {
            let last_head = self.head.clone();
            match direction {
                "R" => self.head.x += 1,
                "L" => self.head.x -= 1,
                "U" => self.head.y += 1,
                "D" => self.head.y -= 1,
                _ => panic!(),
            }
            if self.tail.distance(&self.head) > 1 {
                self.tail = last_head;
                self.visited.insert(self.tail.clone());
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

    let mut rope = Rope::new();
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
    fn test_move_right() {
        let mut rope = Rope::new();

        rope.travel("R", 4);
        let mut expected = Rope {
            head: Point::new(4, 0),
            tail: Point::new(3, 0),
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
            head: Point::new(4, 4),
            tail: Point::new(4, 3),
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
            head: Point::new(1, 4),
            tail: Point::new(2, 4),
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
            head: Point::new(1, 3),
            tail: Point::new(2, 4),
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
