use crate::{
    grid::{Cell, Grid, Node},
    point::{Point, Points},
};
use std::collections::VecDeque;

pub fn bfsearch(grid: &mut Grid<Node>) -> Option<Points> {
    let start = grid.find(Cell::Start).unwrap();
    let end = grid.find(Cell::End).unwrap();

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

            let mut neighbours = grid.get_walkable_neighbours(&current);
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

    fn print(title: &str, path: &Option<Points>) {
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
