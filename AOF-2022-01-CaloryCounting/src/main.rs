use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

struct Counter {
    sums: Vec<u64>,
    current: u64,
}

fn process_line(line: String, counter: Counter) -> Counter {
    let mut new_counter = Counter {
        sums: counter.sums,
        current: counter.current,
    };
    if line.is_empty() {
        // An empty line means we close the current count
        new_counter.sums.push(counter.current);
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
    // Data management
    let mut count = Counter {
        sums: Vec::new(),
        current: 0,
    };

    for (_index, file_line) in reader.lines().enumerate() {
        let line = file_line.unwrap();
        count = process_line(line, count);
    }

    // Sort and sum the top 3
    count.sums.sort_by_key(|n| std::u64::MAX - n);
    count.sums.truncate(3);
    let mut sum = 0;
    for value in count.sums.iter() {
        sum = sum + value;
    }

    println!("Sum of top 3 calories is {}", sum);
}

fn main() {
    // Read the input file
    let filename = "data/input.txt";
    process_input(filename);
}
