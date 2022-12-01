use std::fs::File;
use std::io::{BufRead, BufReader};

struct Counter {
    max: u64,
    max_line: u32,
    current: u64,
}

fn process_line(line: String, line_num: u32, counter: Counter) -> Counter {
    let mut new_counter = Counter {
        max: counter.max,
        max_line: counter.max_line,
        current: counter.current,
    };
    if line.is_empty() {
        // An empty line means we close the current count
        if counter.current > counter.max {
            new_counter.max = counter.current;
            new_counter.max_line = line_num;
        }
        new_counter.current = 0;
    } else {
        // A non-empty line means we accumulate value
        let increment = line.parse::<u64>().unwrap();
        new_counter.current = new_counter.current + increment;
    }
    return new_counter;
}

fn process_input(filename: &str) {
    // File Processing
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut line_num: u32 = 0;
    // Data management
    let mut count = Counter {
        max: 0,
        max_line: 0,
        current: 0,
    };

    for (_index, file_line) in reader.lines().enumerate() {
        let line = file_line.unwrap();
        line_num = line_num + 1;
        count = process_line(line, line_num, count);
    }

    println!("Max count is {} on line {}", count.max, count.max_line);
}

fn main() {
    // Read the input file
    let filename = "data/input.txt";
    process_input(filename);
}
