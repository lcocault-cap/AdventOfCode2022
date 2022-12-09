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

fn is_visible(grid: &Grid, row: usize, col: usize) -> bool {
    let ref_height = grid.trees.get(row).unwrap().get(col).unwrap();
    let mut visible = true;
    // Check from the left
    for i in 0..col {
        let current_height = grid.trees.get(row).unwrap().get(i).unwrap();
        if current_height >= ref_height {
            visible = false;
        }
    }
    if visible {
        return true;
    } else {
        visible = true;
    }
    // Check from the right
    for i in (col + 1)..grid.width {
        let current_height = grid.trees.get(row).unwrap().get(i).unwrap();
        if current_height >= ref_height {
            visible = false;
        }
    }
    if visible {
        return true;
    } else {
        visible = true;
    }
    // Check from the top
    for i in 0..row {
        let current_height = grid.trees.get(i).unwrap().get(col).unwrap();
        if current_height >= ref_height {
            visible = false;
        }
    }
    if visible {
        return true;
    } else {
        visible = true;
    }
    // Check from the bottom
    for i in row + 1..grid.height {
        let current_height = grid.trees.get(i).unwrap().get(col).unwrap();
        if current_height >= ref_height {
            visible = false;
        }
    }
    if visible {
        return true;
    } else {
        return false;
    }
}

fn compute_visible_trees(grid: &Grid) -> u32 {
    let mut count = 0;
    for row in 0..grid.height {
        for col in 0..grid.width {
            if is_visible(grid, row, col) {
                count = count + 1;
            }
        }
    }
    return count;
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

    let visible_trees = compute_visible_trees(&grid);

    println!("Number of visible trees is {}", visible_trees);
}

fn main() {
    // Read the input file
    let filename = "data/input.txt";
    process_input(filename);
}
