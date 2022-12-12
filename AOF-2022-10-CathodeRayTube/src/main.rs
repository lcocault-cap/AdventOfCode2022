use std::fs::File;
use std::io::{BufRead, BufReader};

struct State {
    cycle: u32,
    register: i32,
}

fn execute_instruction(state: &mut State, cycle_count: u32, register_increment: i32) {
    // Process cycles
    for _i in 0..cycle_count {
        // Display the current pixel
        let relative_cycle = (state.cycle % 40) as i32;
        let signal = state.register;
        if relative_cycle >= (signal - 1) && relative_cycle <= (signal + 1) {
            print!("#");
        } else {
            print!(".");
        }
        // New cycle
        state.cycle = state.cycle + 1;
        // Signal update
        if state.cycle > 0 && state.cycle % 40 == 0 {
            println!("");
        }
    }
    // Update the registry
    state.register = state.register + register_increment;
}

fn process_line(line: String, state: &mut State) {
    // Parse lines
    let mut line_split = line.split(" ");
    let instruction = line_split.next().unwrap();

    // Processing arguments
    let mut cycle_count = 0;
    let mut register_increment = 0;

    // Interpret action
    if "addx".eq(instruction) {
        cycle_count = 2;
        register_increment = line_split.next().unwrap().parse::<i32>().unwrap();
    } else if "noop".eq(instruction) {
        cycle_count = 1;
    }

    // Execute the instruction
    execute_instruction(state, cycle_count, register_increment);
}

fn process_input(filename: &str) {
    // File Processing
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // Data management
    let mut state = State {
        cycle: 0,
        register: 1,
    };

    for (_index, file_line) in reader.lines().enumerate() {
        let line = file_line.unwrap();
        process_line(line, &mut state);
    }
}

fn main() {
    // Read the input file
    let filename = "data/input.txt";
    process_input(filename);
}
