use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FILENAME: &str = "input";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut grid: Grid = Vec::new();
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        grid.push(convert_to_heights(&line));
    }

    println!("The grid is {:?}", grid);
}

type Height = isize;
type Heights = Vec<Height>;

struct Grid {
    grid: Vec<Heights>;
}

fn convert_to_heights(line: &String) -> Heights {
    line
        .chars()
        .map(|c| match c {
            'S' => 0,
            'E' => 99,
            _ => (c as usize - 'a' as usize + 1) as isize
        })
        .collect::<Heights>()
}

fn climb(grid: &Grid) {
    // find start
    let (mut x0, mut y0);
    for y in 0..grid.len() {
        for x in  0..grid[0].len() {
            if grid[y][x] == 0 {
                (x0, y0) = (x, y)
            }
        }
    }

    hill_climb(&grid, x0, y0);
}

fn hill_climb(grid: &Grid, x: Height, y: Height) -> Vec<(Height, Height)> {
    if grid[y][x] == 99 {
        return vec![(x, y)];
    } else {
        let coords = [
            (x, y + 1), // up
            (x, y - 1), // down
            (x - 1, y), // right
            (x - 1, y), // left
        ];

        for (x1, y1) in coords.iter() {
            if in_bounds(grid, *x1, *y1) && is_climbable(&grid, *x1, *y1, x, y) {
                hill_climb(

            }
        }

        Vec::new()
    }

}

fn is_climbable(grid: &Grid, x1: Height, y1: Height, x0: Height, y0: Height) -> bool {
    (grid[y1 as usize][x1 as usize] - grid[y0 as usize][x0 as usize]).abs() <= 1
}

fn in_bounds(grid: &Grid, x: Height, y: Height) -> bool {
    if x < 0 || y < 0 || x >= grid[0].len() as isize || y >= grid.len() as isize{
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input_grid() -> Grid {
        vec![
            convert_to_heights(&"Sabqponm".into()),
            convert_to_heights(&"abcryxxl".into()),
            convert_to_heights(&"accszExk".into()),
            convert_to_heights(&"acctuvwj".into()),
            convert_to_heights(&"abdefghi".into()),
        ]
    }

    #[test]
    fn test_hill_climb() {
        let expected: Grid = vec![
            convert_to_heights(&"v..v<<<<".into()),
            convert_to_heights(&">v.vv<<^".into()),
            convert_to_heights(&".>vv>E^^".into()),
            convert_to_heights(&"..v>>>^^".into()),
            convert_to_heights(&"..>>>>>^".into()),
        ];

        assert_eq!(climb(input_grid), expected);
    }
}
