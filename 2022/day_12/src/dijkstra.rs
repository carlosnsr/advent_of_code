use crate::{
    common::get_neighbours,
    grid::{Grid, Node},
    point::{Point, Points},
};
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;

pub fn find_shortest_path(grid: &Grid<Node>) -> Option<Points> {
    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();

    shortest_path(&grid, &start, &end)
}

type DNode = (usize, Option<Point>);
type QNode = (usize, Point);

fn shortest_path(grid: &Grid<Node>, start: &Point, target: &Point) -> Option<Points> {
    let grid_size = grid.len_x() * grid.len_y();
    let mut distances: HashMap<Point, DNode> = HashMap::with_capacity(grid_size);
    let mut queue: BinaryHeap<Reverse<QNode>> = BinaryHeap::new();

    // add start and its neighbours, queue up its neighbours
    distances.insert(start.clone(), (0, None));
    for neighbour in get_neighbours(&grid, &start) {
        distances.insert(neighbour.clone(), (1, Some(start.clone())));
        queue.push(Reverse((1, neighbour.clone())));
    }

    while let Some(Reverse((distance, popped))) = queue.pop() {
        for neighbour in get_neighbours(&grid, &popped) {
            let new_distance = distance + 1;
            match distances.get(&neighbour) {
                // if we've never been here before, add it and queue it
                None => {
                    distances.insert(neighbour.clone(), (new_distance, Some(popped.clone())));
                    queue.push(Reverse((new_distance, neighbour.clone())));
                }
                // if we already found a smaller distance, do nothing
                Some((best_distance, _)) if new_distance >= *best_distance => {},
                // we have found a shorter distance, save it and queue it
                Some(_) => {
                    *(distances.get_mut(&neighbour).unwrap()) = (new_distance, Some(popped.clone()));
                    queue.push(Reverse((new_distance, neighbour.clone())));
                }
            }
        }
    }

    if distances.contains_key(&target) {
        let mut path = Vec::new();
        path.push(target.clone());
        let mut distance_node = distances.get(&target).unwrap();
        while let Some(parent) = &distance_node.1 {
            path.push(parent.clone());
            distance_node = distances.get(&parent).unwrap();
        }
        path.reverse();
        Some(path)
    } else {
        println!("Target {:?} was never reached", &target);
        None
    }
}

#[cfg(test)]
mod tests {
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
        let path = find_shortest_path(&mut grid);
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
        println!("Grid: {:?}", &example_input());
        let path = find_shortest_path(&mut grid);
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
