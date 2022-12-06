use std::fs::File;
use std::io::{BufRead, BufReader};

// Size of the comparison buffer
const BUFF_SIZE: u32 = 4;

fn is_a_marker(buffer: Vec<char>) -> bool {
    let mut copy = buffer.to_vec();
    let mut marker: bool = true;
    while marker && !copy.is_empty() {
        let reference = copy.pop().unwrap();
        marker = !copy.contains(&reference);
    }
    return marker;
}

fn process_line(line: String) -> u32 {
    // Management data
    let mut buffer: Vec<char> = Vec::new();
    let mut chars = line.chars();
    let mut marker: u32 = 0;
    let mut counter: u32 = 0;

    while marker == 0 {
        // Process next character
        let next = chars.next().unwrap();
        counter = counter + 1;

        if counter > BUFF_SIZE {
            // Buffer is full: remove the last item
            buffer.pop();
        }

        // Always add the new character at the beginning
        buffer.insert(0, next);

        // And check the marker
        if counter >= BUFF_SIZE && is_a_marker(buffer.to_vec()) {
            marker = counter;
        }
    }
    return marker;
}

fn process_input(filename: &str) {
    // File Processing
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (_index, file_line) in reader.lines().enumerate() {
        let line = file_line.unwrap();
        let marker = process_line(line);
        println!("Message starts after {} characters", marker);
    }
}

fn main() {
    // Read the input file
    let filename = "data/input.txt";
    process_input(filename);
}
