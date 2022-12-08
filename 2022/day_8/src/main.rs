use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut grid: HeightsGrid = vec![];
    // populates grid
    for (_index, line) in reader.lines().enumerate() {
        grid.push(convert_to_heights(&line.unwrap()));
    }

    let vis_grid: VisGrid = determine_visibility(&grid);
    let visible_trees = count_visible_trees(&vis_grid);
    println!("The number of visible trees is {}", visible_trees);
}

type Heights = Vec<i32>;
type HeightsGrid = Vec<Heights>;
type Visibilities = Vec<bool>;
type VisGrid = Vec<Visibilities>;

fn convert_to_heights(line: &String) -> Heights {
    const RADIX: u32 = 10;
    line
        .chars()
        .map(|s| s.to_digit(RADIX).unwrap() as i32)
        .collect()
}

fn determine_visibility(grid: &HeightsGrid) -> VisGrid {
    let mut vis_grid = make_visibility_grid(grid);

    // examine each row
    for (index, row) in grid.iter().enumerate() {
        mark_row_visibilities(row, &mut vis_grid[index]);
    }

    mark_column_visibilities(&grid, &mut vis_grid);

    vis_grid
}

fn make_visibility_grid(grid: &HeightsGrid) -> VisGrid {
    let max_x = grid[0].len();
    let max_y = grid.len();
    let vis_grid: VisGrid = vec![vec![false; max_x]; max_y];
    vis_grid
}

fn mark_row_visibilities(row: &Heights, vis_map: &mut Visibilities) {
    const MIN_HEIGHT: i32 = -1;

    // visibility from the left
    let mut max_height = MIN_HEIGHT;
    for (index, height) in row.iter().enumerate() {
        if height > &max_height {
            vis_map[index] = true;
            max_height = height.clone();
        }
    }

    // visibility from the right
    // println!("{:?}", heights);
    max_height = MIN_HEIGHT;
    let length = vis_map.len() - 1;
    for (index, height) in row.iter().rev().enumerate() {
        // println!("{:?} {:?}", length - index, height);
        if height > &max_height {
            vis_map[length - index] = true;
            max_height = height.clone();
        }
    }
}

fn mark_column_visibilities(grid: &HeightsGrid, vis_grid: &mut VisGrid) {
    const MIN_HEIGHT: i32 = -1;
    let max_x = grid[0].len();
    let max_y = grid.len();

    // visibility from the top
    let mut max_height;
    for x in 0..max_x {
        max_height = MIN_HEIGHT;
        for y in 0..max_y {
            let height = grid[y][x];
            if height > max_height {
                vis_grid[y][x] = true;
                max_height = height;
            }
        }
    }

    // visibility from the bottom
    let mut max_height;
    for x in (0..max_x).rev() {
        max_height = MIN_HEIGHT;
        for y in (0..max_y).rev() {
            let height = grid[y][x];
            if height > max_height {
                vis_grid[y][x] = true;
                max_height = height;
            }
        }
    }
}

fn count_visible_trees(vis_grid: &VisGrid) -> usize {
    let mut visible = 0;
    for row in vis_grid.iter() {
        for is_visible in row.iter() {
            if *is_visible {
                visible += 1;
            }
        }
    }
    visible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_heights() {
        let input = "13254323".into();
        let expected = vec![1, 3, 2, 5, 4, 3, 2, 3];
        assert_eq!(convert_to_heights(&input), expected);
    }

    #[test]
    fn test_mark_visibilities_by_row() {
        let input = "13254323".into();
        let heights = convert_to_heights(&input);
        let mut vis_map = vec![false; input.len()];
        mark_row_visibilities(&heights, &mut vis_map);
        assert_eq!(
            vis_map,
            [true, true, false, true, true, false, false, true],
        );
        assert_eq!(
            vis_map.iter().fold(0, |acc, visible| acc + (if *visible { 1 } else { 0 })),
            5
        );
    }

    fn input_grid() {
        vec![
            convert_to_heights(&"30373".into()),
            convert_to_heights(&"25512".into()),
            convert_to_heights(&"65332".into()),
            convert_to_heights(&"33549".into()),
            convert_to_heights(&"35390".into()),
        ];
    }

    #[test]
    fn test_mark_visibilities_by_column() {
        let grid = vec![
            vec![3],
            vec![2],
            vec![6],
            vec![3],
            vec![3],
        ];
        let mut vis_grid = make_visibility_grid(&grid);
        mark_column_visibilities(&grid, &mut vis_grid);

        assert_eq!(
            vis_grid,
            [[true], [false], [true], [false], [true]],
        );
    }

    #[test]
    fn test_determine_visibility() {
        let grid = vec![
            convert_to_heights(&"30373".into()),
            convert_to_heights(&"25512".into()),
            convert_to_heights(&"65332".into()),
            convert_to_heights(&"33549".into()),
            convert_to_heights(&"35390".into()),
        ];
        let expected = vec![
            vec![true, true, true, true, true],
            vec![true, true, true, false, true],
            vec![true, true, false, true, true],
            vec![true, false, true, false, true],
            vec![true, true, true, true, true],
        ];

        let actual = determine_visibility(&grid);
        for vis in actual.iter() {
            println!("{:?}", vis);
        }
        println!("");
        for vis in expected.iter() {
            println!("{:?}", vis);
        }
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_count_visible_trees() {
        let vis_grid = vec![
            vec![true, true, true, true, true],
            vec![true, true, true, false, true],
            vec![true, true, false, true, true],
            vec![true, false, true, false, true],
            vec![true, true, true, true, true],
        ];
        assert_eq!(count_visible_trees(&vis_grid), 21);
    }
}
