use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "input";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut grid: Grid = Grid::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        grid.push(&line);
    }

    println!("The grid is {:?}", grid);
}

#[derive(Clone, Debug, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self{ x, y }
    }
}

type Height = usize;
type Heights = Vec<Height>;

#[derive(Debug)]
struct Grid {
    grid: Vec<Heights>,
    visited: Vec<Vec<char>>,
}

impl Grid {
    fn new() -> Self {
        Self { grid: Vec::new(), visited: Vec::new() }
    }

    fn push(&mut self, line: &String) {
        let heights = convert_to_heights(&line);
        self.visited.push(vec!['.'; heights.len()]);
        self.grid.push(heights);
    }

    fn len_x(&self) -> usize {
        self.grid[0].len()
    }

    fn len_y(&self) -> usize {
        self.grid.len()
    }

    fn find(&self, target: Height) -> Option<Coord> {
        for y in 0..self.len_y() {
            for x in 0..self.len_x() {
                if self.grid[y][x] == target {
                    return Some(Coord::new(x, y));
                }
            }
        }
        None
    }

    fn get(&self, target: &Coord) -> Option<Height> {
        let (x, y) = (target.x, target.y);
        Some(self.grid[y][x])
    }

    fn mark(&mut self, target: &Coord, mark: char) {
        let (x, y) = (target.x, target.y);
        self.visited[y][x] = mark;
    }

    fn neighbours(&self, target: &Coord) -> Vec<(char, Coord)> {
        let (x, y) = (target.x, target.y);
        let mut neighbours = Vec::new();
        if x + 1 < self.len_x() {
            neighbours.push(('>', Coord::new(x + 1, y))); // right
        }
        if x > 0 {
            neighbours.push(('>', Coord::new(x - 1, y))); // left
        }
        if y > 0 {
            neighbours.push(('^', Coord::new(x, y - 1))); // up
        }
        if y + 1 < self.len_y() {
            neighbours.push(('v', Coord::new(x, y + 1))); // down
        }

        neighbours
    }

    fn was_visited(&self, target: &Coord) -> bool {
        let (x, y) = (target.x, target.y);
        self.visited[y][x] != '.'
    }
}

fn convert_to_heights(line: &String) -> Heights {
    println!("{:?}", line);
    line
        .chars()
        .map(|c| match c {
            'S' => 0,
            'E' => 99,
            _ => c as usize - 'a' as usize + 1
        })
        .collect::<Heights>()
}

fn climb(grid: &Grid) -> Option<Vec<Coord>> {
    let start = grid.find(0).unwrap();
    println!("Found {:?}", start);
    let end = grid.find(99).unwrap();
    println!("Found {:?}", end);

    hill_climb(&mut grid, &start, &end)
}

fn hill_climb(grid: &mut Grid, target: &Coord, end: &Coord) -> Option<Vec<Coord>> {
    if target == end {
        Some(vec![target.clone()])
    } else {
        let neighbours = grid.neighbours(&target);
        println!("{:?}", neighbours);
        for (mark, neighbour) in neighbours.iter() {
            if is_climbable(&grid, target, neighbour) {
                grid.mark(target, mark.clone());
                match hill_climb(&mut grid, &neighbour, &end) {
                    Some(mut path) => {
                        path.push(neighbour.clone());
                        return Some(path);
                    },
                    None => (),
                }
            }
        }

        None
    }
}

fn is_climbable(grid: &Grid, source: &Coord, target: &Coord) -> bool {
    let source_height = grid.get(&source).unwrap() as isize;
    let target_height = grid.get(&target).unwrap() as isize;
    println!("{:?} {:?}", source_height, target_height);

    target_height == 99 || (source_height - target_height).abs() <= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input_grid() -> Grid {
        let lines = vec![
            "Sabqponm".into(),
            "abcryxxl".into(),
            "accszExk".into(),
            "acctuvwj".into(),
            "abdefghi".into(),
        ];
        let mut grid = Grid::new();
        for line in lines.iter() {
            grid.push(&line);
        }
        grid
    }

    #[ignore]
    #[test]
    fn test_hill_climb() {
        let expected: Vec<String> = vec![
            "v..v<<<<".into(),
            ">v.vv<<^".into(),
            ".>vv>E^^".into(),
            "..v>>>^^".into(),
            "..>>>>>^".into(),
        ];

        let paths = climb(&input_grid());
        println!("{:?}", paths);
        assert!(false);
        // assert_eq!(climb(input_grid), expected);
    }

    #[test]
    fn test_little_hill_climbs() {
        let mut grid = Grid::new();
        grid.push(&"Sa".into());
        grid.push(&"bE".into());
        println!("{:?}", grid);
        climb(&grid);
        println!("{:?}", grid);
        assert!(false);
    }
}
