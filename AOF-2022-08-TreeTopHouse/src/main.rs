use std::fs::File;
use std::io::{BufRead, BufReader};

struct Grid {
    width: usize,
    height: usize,
    trees: Vec<Vec<u32>>,
}

fn process_line(line: String, grid: &mut Grid) {
    // Update the grid dimensions for further computations
    if grid.width == 0 {
        grid.width = line.len();
    }
    grid.height = grid.height + 1;

    // Read the line and feed the grid
    grid.trees.insert(0, Vec::new());
    for tree in line.chars() {
        let tree_height = tree as u32;
        grid.trees.get_mut(0).unwrap().insert(0, tree_height - 48);
    }
}

fn compute_score(grid: &Grid, row: usize, col: usize) -> u32 {
    let ref_height = grid.trees.get(row).unwrap().get(col).unwrap();
    // Check from the left
    let mut left_score = 0;
    for i in (0..col).rev() {
        let current_height = grid.trees.get(row).unwrap().get(i).unwrap();
        left_score = left_score + 1;
        if current_height >= ref_height {
            break;
        }
    }
    // Check from the right
    let mut right_score = 0;
    for i in (col+1)..grid.width {
        let current_height = grid.trees.get(row).unwrap().get(i).unwrap();
        right_score = right_score + 1;
        if current_height >= ref_height {
            break;
        }
    }

    // Check from the top
    let mut top_score = 0;
    for i in (0..row).rev() {
        let current_height = grid.trees.get(i).unwrap().get(col).unwrap();
        top_score = top_score + 1;
        if current_height >= ref_height {
            break;
        }
    }
    // Check from the bottom
    let mut bottom_score = 0;
    for i in (row + 1)..grid.height {
        let current_height = grid.trees.get(i).unwrap().get(col).unwrap();
        bottom_score = bottom_score + 1;
        if current_height >= ref_height {
            break;
        }
    }

    return left_score * right_score * bottom_score * top_score;
}

fn compute_best_scenic_score(grid: &Grid) -> u32 {
    let mut best_score = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            let score = compute_score(grid, row, col);
            if score > best_score {
                best_score = score;
            }
        }
    }
    return best_score;
}

fn process_input(filename: &str) {
    // File Processing
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Management data
    let mut grid = Grid {
        width: 0,
        height: 0,
        trees: Vec::new(),
    };

    for (_index, file_line) in reader.lines().enumerate() {
        let line = file_line.unwrap();
        process_line(line, &mut grid);
    }

    let best_scenic_score = compute_best_scenic_score(&grid);

    println!("Best scenic score is {}", best_scenic_score);
}

fn main() {
    // Read the input file
    let filename = "data/input.txt";
    process_input(filename);
}
