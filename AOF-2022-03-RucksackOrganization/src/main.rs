use std::fs::File;
use std::io::{BufRead, BufReader};

fn process_line(line: String, sum: i32) -> i32 {
    // Get the content of the compartments
    let size = line.len();
    let (first_compartment, second_compartment) = line.split_at(size / 2);

    // Find the common item
    let mut common_item = ' ';
    for item in first_compartment.chars() {
        if second_compartment.contains(item) {
            common_item = item;
        }
    }

    // Compute the value of the common item
    let mut value = 0;
    if ('a'..'{').contains(&common_item) {
        value = common_item as i32 - 'a' as i32 + 1;
    } else if ('A'..'[').contains(&common_item) {
        value = common_item as i32 - 'A' as i32 + 27;
    }

    return sum + value;
}

fn process_input(filename: &str) {
    // File Processing
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // Data management
    let mut sum: i32 = 0;

    for (_index, file_line) in reader.lines().enumerate() {
        let line = file_line.unwrap();
        sum = process_line(line, sum);
    }

    println!("Sum is {}", sum);
}

fn main() {
    // Read the input file
    let filename = "data/input.txt";
    process_input(filename);
}
