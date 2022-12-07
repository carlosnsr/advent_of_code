use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input";

#[derive(Debug)]
struct Solver {
    sum: usize,
    map: HashMap<String, Vec<usize>>,
    path: VecDeque<String>,
}

impl Solver {
    fn new() -> Self {
        Self {
            sum: 0,
            map: HashMap::new(),
            path: VecDeque::new(),
        }
    }

    fn insert(&mut self, name: String) {
        match self.map.get_mut(&name) {
            Some(sizes) => { sizes.push(0); }
            None => { self.map.insert(name, vec![0]); }
        }
    }

    fn cd(&mut self, name: String) {
        self.path.push_back(name);
    }

    fn add(&mut self, size: usize) {
        let name = self.path.back().unwrap().clone();
        if let Some(dirs) = self.map.get_mut(&name) {
            let mut curr_size = dirs.pop().unwrap();
            curr_size += size;
            dirs.push(curr_size);
        }
    }

    fn pop(&mut self) {
        let name = self.path.pop_back().unwrap();
        // println!("{:?} dropping {:?}", index, name);
        if let Some(dirs) = self.map.get_mut(&name) {
            let final_size = dirs.pop().unwrap();
            if final_size <= 100_000 {
                self.sum += final_size;
            }

            // update parent's size
            let parent_name = self.path.back().unwrap().clone();
            if let Some(parent) = self.map.get_mut(&parent_name) {
                let mut parent_size = parent.pop().unwrap();
                parent_size += final_size;
                parent.push(parent_size);
            }
        }

        // drop if empty
        if let Some(dirs) = self.map.get(&name) {
            if dirs.is_empty() {
                self.map.remove(&name);
            }
        }
    }

    fn flush(&mut self) {
        while self.path.len() > 1 {
            self.pop();
        }
    }
}

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut solver = Solver::new();
    solver.insert("/".into());

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        match Command::parse(&line) {
            Command::Chdir(name) => { solver.cd(name); },
            Command::List => (),
            Command::Dir(name) => { solver.insert(name); },
            Command::File(size) => { solver.add(size); },
            Command::Up => { solver.pop(); }
            _ => println!("{:?}", line),
        }
        // if index > 32 { break; }
    }
    solver.flush();
    println!("{:?}", solver);
}

#[derive(Debug, PartialEq)]
enum Command {
    Up,
    Chdir(String),
    List,
    Dir(String),
    File(usize),
}

impl Command {
    fn parse(line: &String) -> Command {
        match line.as_str() {
            "$ cd .." => Command::Up,
            "$ ls" => Command::List,
            _ => {
                let tokens: Vec<&str> = line.split(" ").collect();
                match tokens[0] {
                    "$" => Command::Chdir(tokens[2].into()),
                    "dir" => Command::Dir(tokens[1].into()),
                    _ => Command::File(tokens[0].parse().unwrap()),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_returns_commands() {
        assert_eq!(Command::parse(&"$ cd ..".into()), Command::Up);
        assert_eq!(Command::parse(&"$ cd /".into()), Command::Chdir("/".into()));
        assert_eq!(Command::parse(&"$ ls".into()), Command::List);
        assert_eq!(Command::parse(&"dir a".into()), Command::Dir("a".into()));
        assert_eq!(Command::parse(&"1000 b.txt".into()), Command::File(1000));
    }
}
