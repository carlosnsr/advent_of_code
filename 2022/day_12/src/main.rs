use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "input";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut grid: Grid<Node> = Grid::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        grid.push(&line);
    }

    println!("The grid is {:?}", grid);
    if let Some(path) = bfsearch(&mut grid) {
        println!("The shortestt number of steps is {}", path.len() - 1);
    } else {
        println!("No path was found");
    }
}

type Height = char;
type Heights = Vec<Height>;
type Points = Vec<Point>;
type Path = Vec<Point>;

trait Newable {
    fn new(value: Height) -> Self;
}

trait Valuable {
    fn value(&self) -> &Height;
}

#[derive(Debug)]
struct Node {
    value: Height,
    visited: bool,
    parent: Option<Point>,
}

impl Newable for Node {
    fn new(value: Height) -> Self {
        Self {
            value,
            visited: false,
            parent: None,
        }
    }
}

impl Valuable for Node {
    fn value(&self) -> &Height {
        &self.value
    }
}

#[derive(Debug)]
struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl<T: Newable + Valuable> Grid<T> {
    fn new() -> Self {
        Self { grid: Vec::new() }
    }

    fn push(&mut self, line: &String) {
        let row: Vec<T> = line.chars().map(|c| T::new(c)).collect();
        self.grid.push(row);
    }

    fn find(&self, value: Height) -> Option<Point> {
        for y in 0..self.len_y() {
            for x in 0..self.len_x() {
                if *self.grid[y][x].value() == value {
                    return Some(Point::new(x, y));
                }
            }
        }
        None
    }

    fn get_mut(&mut self, target: &Point) -> Option<&mut T> {
        let (x, y) = (target.x, target.y);
        Some(&mut self.grid[y][x])
    }

    fn get(&self, target: &Point) -> Option<&T> {
        let (x, y) = (target.x, target.y);
        Some(&self.grid[y][x])
    }

    fn len_x(&self) -> usize {
        self.grid[0].len()
    }

    fn len_y(&self) -> usize {
        self.grid.len()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self{ x, y }
    }

    fn distance(&self, other: &Point) -> f32 {
        let (dx, dy): (f32, f32) = ((other.x as isize - self.x as isize) as f32, (other.y as isize - self.y as isize) as f32);

        (dx * dx + dy * dy).sqrt()
    }
}

fn bfsearch(grid: &mut Grid<Node>) -> Option<Points> {
    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();

    let mut root = grid.get_mut(&start).unwrap();
    let mut queue: VecDeque<Point> = VecDeque::new();
    root.visited = true;
    queue.push_back(start.clone());
    while !queue.is_empty() {
        if let Some(current) = queue.pop_front() {
            if current == end {
                let mut path: Points = Vec::new();
                path.push(current.clone());
                let mut node = grid.get(&current).unwrap();
                while let Some(parent) = &node.parent {
                    path.push(parent.clone());
                    node = grid.get(&parent).unwrap();
                }
                path.reverse();
                return Some(path);
            }

            let mut neighbours = find_neighbours(&grid, &current);
            for neighbour in neighbours.drain(0..) {
                if let Some(node) = grid.get_mut(&neighbour) {
                    if node.visited {
                        continue;
                    }
                    node.visited = true;
                    node.parent = Some(current.clone());
                    queue.push_back(neighbour);
                }
            }
        }
    }

    None
}

fn find_neighbours(grid: &Grid<Node>, center: &Point) -> Points {
    let (x, y) = (center.x, center.y);
    let mut neighbours = Vec::new();
    if x + 1 < grid.len_x() { // right
        neighbours.push(Point::new(x + 1, y));
    }
    if x > 0 { // left
        neighbours.push(Point::new(x - 1, y));
    }
    if y > 0 { // up
        neighbours.push(Point::new(x, y - 1));
    }
    if y + 1 < grid.len_y() { // down
        neighbours.push(Point::new(x, y + 1));
    }

    let mut vetted = Vec::new();
    for neighbour in neighbours.drain(..) {
        let center_height = height(grid.get(&center).unwrap().value) as isize;
        if is_vetted(&grid, center_height, &neighbour) {
            vetted.push(neighbour)
        }
    }
    // println!("   Vetted Neighbours {:?}", &vetted);
    vetted
}

fn is_vetted(grid: &Grid<Node>, center_height: isize, neighbour: &Point) -> bool {
    if let Some(node) = grid.get(neighbour) {
        if node.visited {
            return false;
        }
    }

    let neighbour_height = height(grid.get(&neighbour).unwrap().value) as isize;
    let diff = (center_height - neighbour_height).abs();
    // println!("   Comparing with {:?} (height: {}): {}", neighbour, neighbour_height, diff);
    if diff > 1 {
        false
    } else {
        true
    }
}

fn height(value: Height) -> usize {
    match value {
        'S' => 0, // should match the height of 'a'
        'E' => 'z' as usize - 'a' as usize, // should match the height of 'z'
        _ => value as usize - 'a' as usize,
    }
}

/*
fn climb_to_top(grid: &Grid) -> Option<Path> {
    let start = grid.find('S').unwrap();
    println!("Start position is: {:?}", &start);
    let end = grid.find('E').unwrap();
    println!("End position is: {:?}", &end);
    let mut visited: HashSet<Point> = HashSet::new();

    match hill_climb(&grid, &start, &end, &mut visited) {
        Some(mut path) => {
            path.push(start.clone());
            path.reverse();
            Some(path)
        },
        None => None,
    }
}

fn hill_climb(grid: &Grid, current: &Point, end: &Point, visited: &mut HashSet<Point>) -> Option<Vec<Point>> {
    println!("Calling hill_climb with {:?}", current);
    visited.insert(current.clone());
    if current == end {
        return Some(vec![]);
    } else {
        let neighbours = find_neighbours(grid, current, &visited);
        let neighbours_by_distance = get_neighbours_by_distance(neighbours, &end);
        for i in 0..neighbours_by_distance.len() {
            let (_, neighbour) = &neighbours_by_distance[i];
            if visited.contains(&neighbour) {
                continue;
            }
            match hill_climb(&grid, neighbour, &end, visited) {
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

fn find_neighbours(grid: &Grid, center: &Point, visited: &HashSet<Point>) -> Neighbours {
    let (x, y) = (center.x, center.y);
    let mut neighbours = Vec::new();
    if x + 1 < grid.len_x() { // right
        neighbours.push(Point::new(x + 1, y));
    }
    if x > 0 { // left
        neighbours.push(Point::new(x - 1, y));
    }
    if y > 0 { // up
        neighbours.push(Point::new(x, y - 1));
    }
    if y + 1 < grid.len_y() { // down
        neighbours.push(Point::new(x, y + 1));
    }

    let mut vetted = Vec::new();
    for neighbour in neighbours.drain(..) {
        if is_vetted(&grid, &center, &neighbour, &visited) {
            vetted.push(neighbour)
        }
    }
    // println!("   Vetted Neighbours {:?}", &vetted);
    vetted
}

fn is_vetted(grid: &Grid, center: &Point, neighbour: &Point, visited: &HashSet<Point>) -> bool {
    if visited.contains(&neighbour) {
        return false;
    }

    let center_height = height(grid.get(&center).unwrap()) as isize;
    let neighbour_height = height(grid.get(&neighbour).unwrap()) as isize;
    let diff = (center_height - neighbour_height).abs();
    println!("   Comparing {:?} (height: {}) with {:?} (height: {}): {}", &center, center_height, neighbour, neighbour_height, diff);
    if diff > 1 {
        false
    } else {
        true
    }
}

type NeighboursByDistance = Vec<(f32, Point)>;

fn get_neighbours_by_distance(mut neighbours: Neighbours, end: &Point) -> NeighboursByDistance {
    let mut neighbours_by_distance: NeighboursByDistance = neighbours
        .drain(0..)
        .map(|neighbour| (neighbour.distance(&end), neighbour))
        .collect();
    println!("   Neighbours by Distance (unsorted) {:?}", &neighbours_by_distance);
    neighbours_by_distance.sort_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap());
    println!("   Neighbours by Distance (sorted) {:?}", &neighbours_by_distance);
    neighbours_by_distance
}

fn height(value: Height) -> usize {
    match value {
        'S' => 0, // TODO: should match 'a'
        'E' => 'z' as usize - 'a' as usize, // TODO: should match 'z'
        _ => value as usize - 'a' as usize,
    }
}
*/
/*
impl Grid {
    fn get(&self, target: &Point) -> Option<Height> {
        let (x, y) = (target.x, target.y);
        Some(self.grid[y][x])
    }

    fn mark(&mut self, target: &Point, mark: char) {
        let (x, y) = (target.x, target.y);
        self.visited[y][x] = mark;
    }

    fn was_visited(&self, target: &Point) -> bool {
        let (x, y) = (target.x, target.y);
        self.visited[y][x] != '.'
    }
}

fn is_climbable(grid: &Grid, source: &Point, target: &Point) -> bool {
    let source_height = grid.get(&source).unwrap() as isize;
    let target_height = grid.get(&target).unwrap() as isize;
    println!("{:?} {:?}", source_height, target_height);

    target_height == 99 || (source_height - target_height).abs() <= 1
}
*/

#[cfg(test)]
mod bfs_tests {
    use super::*;

    fn make_grid(lines: &Vec<String>) -> Grid<Node> {
        let mut grid = Grid::new();
        for line in lines.iter() {
            grid.push(&line);
        }
        grid
    }

    fn example_input() -> Vec<String> {
        vec![
            "Sabqponm".into(),
            "abcryxxl".into(),
            "accszExk".into(),
            "acctuvwj".into(),
            "abdefghi".into(),
        ]
    }

    fn print(title: &str, path: &Option<Path>) {
        if let Some(path) = path {
            println!("{} ({}):", title, path.len());
            for point in path.iter() {
                println!("   {:?}", point);
            }
        } else {
            println!("{}: None", title);
        }
    }

    #[test]
    fn test_on_simple_grid() {
        let input = vec![
            "Sabcde".into(),
            "yzEdgf".into(),
            "xwvutg".into(),
            "opqrsh".into(),
            "nmlkji".into(),
        ];
        let mut grid = make_grid(&input);
        let path = bfsearch(&mut grid);
        assert_eq!(path, Some(vec![
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(2, 0),
            Point::new(3, 0),
            Point::new(4, 0),
            Point::new(5, 0),
            Point::new(5, 1),
            Point::new(5, 2),
            Point::new(5, 3),
            Point::new(5, 4),

            Point::new(4, 4),
            Point::new(3, 4),
            Point::new(2, 4),
            Point::new(1, 4),
            Point::new(0, 4),

            Point::new(0, 3),
            Point::new(1, 3),
            Point::new(2, 3),
            Point::new(3, 3),
            Point::new(4, 3),

            Point::new(4, 2),
            Point::new(3, 2),
            Point::new(2, 2),
            Point::new(1, 2),
            Point::new(0, 2),

            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(2, 1),
        ]));
    }

    #[test]
    fn test_on_example() {
        // let expected: Vec<String> = vec![
        //     "v..v<<<<".into(),
        //     ">v.vv<<^".into(),
        //     ".>vv>E^^".into(),
        //     "..v>>>^^".into(),
        //     "..>>>>>^".into(),
        // ];
        let expected = Some(vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(1, 2),
            Point::new(2, 2),
            Point::new(2, 3),
            Point::new(2, 4),
            Point::new(3, 4),
            Point::new(4, 4),
            Point::new(5, 4),
            Point::new(6, 4),
            Point::new(7, 4),
            Point::new(7, 3),
            Point::new(7, 2),
            Point::new(7, 1),
            Point::new(7, 0),
            Point::new(6, 0),
            Point::new(5, 0),
            Point::new(4, 0),
            Point::new(3, 0),
            Point::new(3, 1),
            Point::new(3, 2),
            Point::new(3, 3),
            Point::new(4, 3),
            Point::new(5, 3),
            Point::new(6, 3),
            Point::new(6, 2),
            Point::new(6, 1),
            Point::new(5, 1),
            Point::new(4, 1),
            Point::new(4, 2),
            Point::new(5, 2),
        ]);
        let mut grid = make_grid(&example_input());
        let path = bfsearch(&mut grid);
        print("Path", &path);
        print("Expected", &expected);
        // assert_eq!(path, expected);
        if let Some(path) = path {
            if let Some(expected) = expected {
                assert_eq!(path.len(), expected.len());
            } else {
                assert!(false, "Expected is None")
            }
        } else {
            assert!(false, "Path is None")
        }
    }
}

/*
#[cfg(test)]
mod tests {
    #[ignore]
    #[test]
    fn test_hill_climb() {
        // let expected: Vec<String> = vec![
        //     "v..v<<<<".into(),
        //     ">v.vv<<^".into(),
        //     ".>vv>E^^".into(),
        //     "..v>>>^^".into(),
        //     "..>>>>>^".into(),
        // ];
        let expected = Some(vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(1, 2),
            Point::new(2, 2),
            Point::new(2, 3),
            Point::new(2, 4),
            Point::new(3, 4),
            Point::new(4, 4),
            Point::new(5, 4),
            Point::new(6, 4),
            Point::new(7, 4),
            Point::new(7, 3),
            Point::new(7, 2),
            Point::new(7, 1),
            Point::new(7, 0),
            Point::new(6, 0),
            Point::new(5, 0),
            Point::new(4, 0),
            Point::new(3, 0),
            Point::new(3, 1),
            Point::new(3, 2),
            Point::new(3, 3),
            Point::new(4, 3),
            Point::new(5, 3),
            Point::new(6, 3),
            Point::new(6, 2),
            Point::new(6, 1),
            Point::new(5, 1),
            Point::new(4, 1),
            Point::new(4, 2),
            Point::new(5, 2),
        ]);
        let grid = make_grid(&example_input());
        let path = climb_to_top(&grid);
        print("Path", &path);
        print("Expected", &expected);
        // assert_eq!(path, expected);
        if let Some(path) = path {
            if let Some(expected) = expected {
                assert_eq!(path.len(), expected.len());
            } else {
                assert!(false, "Expected is None")
            }
        } else {
            assert!(false, "Path is None")
        }
    }

    #[ignore]
    #[test]
    fn test_little_hill_climbs_2x2() {
        let input = vec![
            "Sa".into(),
            "bE".into(),
        ];
        let grid = make_grid(&input);
        let path = climb_to_top(&grid);
        assert_eq!(path, Some(vec![Point::new(0, 0), Point::new(1, 0), Point::new(1, 1)]));
    }

    #[ignore]
    #[test]
    fn test_little_hill_climbs_3x3() {
        let input = vec![
            "Sac".into(),
            "bbd".into(),
            "dcE".into(),
        ];
        let grid = make_grid(&input);
        let path = climb_to_top(&grid);
        assert_eq!(
            path,
            Some(vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(1, 1),
                Point::new(1, 2),
                Point::new(2, 2),
            ])
        );
    }

    #[ignore]
    #[test]
    fn test_little_hill_climbs_4x4() {
        let input = vec![
            "Sabc".into(),
            "abcd".into(),
            "bcde".into(),
            "cEef".into(),
        ];
        let expected = Some(vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(0, 2),
            Point::new(1, 2),
            Point::new(1, 3),
        ]);
        let grid = make_grid(&input);
        let path = climb_to_top(&grid);
        print("Path", &path);
        print("Expected", &expected);
        assert_eq!(path, expected);
    }
}
*/
