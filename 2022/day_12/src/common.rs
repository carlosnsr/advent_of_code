use crate::{
    grid::{Grid, Node},
    point::{Point, Points},
};

pub type Height = char;

fn calculate_height(value: Height) -> usize {
    match value {
        'S' => 0, // should match the height of 'a'
        'E' => 'z' as usize - 'a' as usize, // should match the height of 'z'
        _ => value as usize - 'a' as usize,
    }
}

pub fn get_neighbours(grid: &Grid<Node>, target: &Point) -> Points {
    let (x, y) = (target.x, target.y);
    let mut neighbour_points = Vec::new();
    if x + 1 < grid.len_x() { // right
        neighbour_points.push(Point::new(x + 1, y));
    }
    if x > 0 { // left
        neighbour_points.push(Point::new(x - 1, y));
    }
    if y > 0 { // up
        neighbour_points.push(Point::new(x, y - 1));
    }
    if y + 1 < grid.len_y() { // down
        neighbour_points.push(Point::new(x, y + 1));
    }

    let node = grid.get(&target).unwrap();
    let height = calculate_height(node.value) as isize;
    let mut vetted_neighbours = Vec::new();
    for neighbour_point in neighbour_points.drain(..) {
        let neighbour = grid.get(&neighbour_point).unwrap();
        let neighbour_height = calculate_height(neighbour.value) as isize;
        if (height - neighbour_height).abs() > 1 {
            continue;
        } else {
            vetted_neighbours.push(neighbour_point);
        }
    }
    // println!("   Vetted Neighbours {:?}", &vetted);
    vetted_neighbours
}
