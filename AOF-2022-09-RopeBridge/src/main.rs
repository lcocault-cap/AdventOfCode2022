use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

// Number of knots on the rope
const NB_KNOTS: usize = 10;

struct State {
    x: [i32; NB_KNOTS],
    y: [i32; NB_KNOTS],
    positions: Vec<i32>,
}

fn perform_action(state: &mut State, delta_x: i32, delta_y: i32) {
    // Update the position of the head with the delta
    state.x[0] = state.x[0] + delta_x;
    state.y[0] = state.y[0] + delta_y;
    // Compute the position of the following knots
    for knot in 1..NB_KNOTS {
        // Compute the delta between two nodes
        let gap_x = state.x[knot - 1] - state.x[knot];
        let gap_y = state.y[knot - 1] - state.y[knot];
        // Move accordingly
        if gap_x.abs() >= 2 || gap_y.abs() >= 2 {
            if gap_x >= 1 {
                state.x[knot] = state.x[knot] + 1;
            }
            if gap_x <= -1 {
                state.x[knot] = state.x[knot] - 1;
            }
            if gap_y >= 1 {
                state.y[knot] = state.y[knot] + 1;
            }
            if gap_y <= -1 {
                state.y[knot] = state.y[knot] - 1;
            }
        }
    }

    // Compute a hash of the tail position and add it to the list of positions if not already explored
    let hash = state.x[NB_KNOTS - 1] * 1000 + state.y[NB_KNOTS - 1];
    if !state.positions.contains(&hash) {
        state.positions.insert(0, hash);
    }
}

fn process_line(line: String, state: &mut State) {
    // Parse lines
    let mut line_split = line.split(" ");
    let action = line_split.next().unwrap();
    let action_count = line_split.next().unwrap().parse::<u32>().unwrap();

    // Interpret action
    let mut delta_x = 0;
    let mut delta_y = 0;
    if "L".eq(action) {
        delta_x = -1;
    } else if "R".eq(action) {
        delta_x = 1;
    } else if "U".eq(action) {
        delta_y = 1;
    } else if "D".eq(action) {
        delta_y = -1;
    }

    // Perform the action several times
    for _i in 0..action_count {
        perform_action(state, delta_x, delta_y);
    }
}

fn process_input(filename: &str) {
    // File Processing
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // Data management
    let mut state = State {
        x: [0; NB_KNOTS],
        y: [0; NB_KNOTS],
        positions: Vec::new(),
    };

    for (_index, file_line) in reader.lines().enumerate() {
        let line = file_line.unwrap();
        process_line(line, &mut state);
    }

    println!("Count of distinct positions is {}", state.positions.len());
}

fn main() {
    // Read the input file
    let filename = "data/input.txt";
    process_input(filename);
}
