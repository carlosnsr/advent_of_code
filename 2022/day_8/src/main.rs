use std::fs::File;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "input";

fn main() {
    let file = File::open(FILENAME).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut grid: Vec<String> = vec![];
    // populates grid
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        grid.push(line);
    }
}

type VisGrid = Vec<Vec<bool>>;

fn determine_visibility(grid: &Vec<String>) -> VisGrid {
    // initialise vis_map
    let mut vis_grid: VisGrid = vec![
        vec![false; grid[0].len()];
        grid.len()
    ];

    for (index, row) in grid.iter().enumerate() {
        mark_visibles(row.as_ref(), &mut vis_grid[index]);
    }

    vis_grid
}

fn mark_visibles(row: &str, vis_map: &mut Vec<bool>) {
    const RADIX: u32 = 10;
    const MIN_HEIGHT: i32 = -1;

    let heights: Vec<i32> = row.chars().map(|s| s.to_digit(RADIX).unwrap() as i32).collect();
    let mut max_height = MIN_HEIGHT;
    // visibility from the left
    for (index, height) in heights.iter().enumerate() {
        if height > &max_height {
            vis_map[index] = true;
            max_height = height.clone();
        }
    }
    // visibility from the left
    // println!("{:?}", heights);
    let length = vis_map.len() - 1;
    max_height = MIN_HEIGHT;
    for (index, height) in heights.iter().rev().enumerate() {
        // println!("{:?} {:?}", length - index, height);
        if height > &max_height {
            vis_map[length - index] = true;
            max_height = height.clone();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        vec![
            "30373".into(),
            "25512".into(),
            "65332".into(),
            "33549".into(),
            "35390".into(),
        ]
    }

    #[test]
    fn test_row_traversal() {
        let input = "13254323";
        let mut vis_map = vec![false; input.len()];
        mark_visibles(&input, &mut vis_map);
        assert_eq!(
            vis_map,
            [true, true, false, true, true, false, false, true],
        );
        assert_eq!(
            vis_map.iter().fold(0, |acc, visible| acc + (if *visible { 1 } else { 0 })),
            5
        );
    }

    #[test]
    fn test_determine_visibility() {
        let grid = input();
        let vis_grid = determine_visibility(&grid);
        for vis in vis_grid.iter() {
            println!("{:?}", vis);
        }
        assert!(false);
    }
}
