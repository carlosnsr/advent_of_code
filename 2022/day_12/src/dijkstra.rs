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
    let mut visited: HashSet<Point> = HashSet::with_capacity(grid_size);

    // add start
    distances.insert(start.clone(), (0, None));
    queue.push(Reverse((0, start.clone())));

    while !queue.is_empty() {
        let Reverse((dist, current)) = queue.pop().unwrap();
        visited.insert(current.clone());
        // println!("\n\nCurrent: {:?}", current);

        let mut neighbours = get_neighbours(&grid, &current);
        // println!("Neighbours: {:?}", neighbours);
        for neighbour in neighbours.drain(0..) {
            let new_dist = dist + 1;
            let distance_node = distances.entry(neighbour.clone())
                .or_insert((new_dist, Some(current.clone())));
            if new_dist < distance_node.0 {
                *distance_node = (new_dist, Some(current.clone()));
            }
            if !visited.contains(&neighbour) {
                queue.push(Reverse((new_dist, neighbour.clone())));
            }
        }
        // println!("Distances\n   {:?}", distances);
        // println!("Queue\n   {:?}", queue);
        // println!("Visited\n   {:?}", visited);
    }

    if visited.contains(&target) {
        let mut distance_node = distances.get(&target).unwrap();
        let mut path = Vec::new();
        path.push(target.clone());
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
