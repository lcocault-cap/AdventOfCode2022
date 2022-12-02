use std::fs::File;
use std::io::{BufRead, BufReader};

struct Position {
    x: u32,
    depth: u32,
}

fn process_line(line: String, position: Position) -> Position {
    let mut new_position = Position {
        x: position.x,
        depth: position.depth,
    };
    // Parse the line as a command
    let mut line_tokens = line.split(" ");
    let command = line_tokens.next().unwrap();
    let value: u32 = line_tokens.next().unwrap().parse::<u32>().unwrap();
    // Interpret the command
    if "forward".eq(command) {
        new_position.x = position.x + value;
    }
    if "down".eq(command) {
        new_position.depth = position.depth + value;
    }
    if "up".eq(command) {
        new_position.depth = position.depth - value;
    }
    return new_position;
}

fn process_input(filename: &str) {
    // File Processing
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // Data management
    let mut position = Position {
        x: 0,
        depth: 0,
    };

    for (_index, file_line) in reader.lines().enumerate() {
        let line = file_line.unwrap();
        position = process_line(line, position);
    }

    println!("Position is {}", position.x*position.depth);
}

fn main() {
    // Read the input file
    let filename = "data/input.txt";
    process_input(filename);
}
