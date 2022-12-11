use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

struct State {
    head_x: i32,
    head_y: i32,
    tail_x: i32,
    tail_y: i32,
    positions: Vec<i32>,
}

fn too_far(state: &mut State) -> bool {
    let delta_x = (state.head_x - state.tail_x).abs();
    let delta_y = (state.head_y - state.tail_y).abs();
    return delta_x > 1 || delta_y > 1;
}

fn perform_action(state: &mut State, delta_x: i32, delta_y: i32) {
    // Previous head position
    let previous_head_x = state.head_x;
    let previous_head_y = state.head_y;
    // Update the position of the head with the delta
    state.head_x = state.head_x + delta_x;
    state.head_y = state.head_y + delta_y;
    // With a rope of length 1, the position of the tail is the previous position of the head
    if too_far(state) {
        state.tail_x = previous_head_x;
        state.tail_y = previous_head_y;
    }
    // Compute a hash of the tail position and add it to the list of positions if not already explored
    let hash = state.tail_x * 1000 + state.tail_y;
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
        head_x: 0,
        head_y: 0,
        tail_x: 0,
        tail_y: 0,
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
