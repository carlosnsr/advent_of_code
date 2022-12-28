use crate::{
    grid::{Cell, Grid, Node, Valuable},
    point::{Point, Points},
};

pub fn get_neighbours(grid: &Grid<Node>, target: &Point) -> Points {
    let target_height = grid.get(target).unwrap().value().height();
    let deltas: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    deltas
        .into_iter()
        .filter_map(move |(dx, dy)|
            Some(Point {
                x: target.x.checked_add_signed(dx)?,
                y: target.y.checked_add_signed(dy)?,
            })
        )
        .filter(|point| grid.in_bounds(&point))
        .filter(|point| {
            let height = grid.get(&point).unwrap().value().height();
            height <= target_height + 1
        })
        .collect()
}
