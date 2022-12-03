use std::fs::File;
use std::io::{BufRead, BufReader};

struct Context {
    sum: u32,
    first: String,
    second: String,
}

// Compute the value of the common item
fn compute_priority(item: char) -> u32 {
    let mut priority = 0;
    if ('a'..'{').contains(&item) {
        priority = item as u32 - 'a' as u32 + 1;
    } else if ('A'..'[').contains(&item) {
        priority = item as u32 - 'A' as u32 + 27;
    }
    return priority;
}

fn process_line(line: String, context: Context) -> Context {
    let mut new_context = Context {
        sum: context.sum,
        first: context.first,
        second: context.second,
    };

    // First line of a group, just store it
    if new_context.first.is_empty() {
        new_context.first = String::from(line);
    }
    // Second line of a group, just store it
    else if new_context.second.is_empty() {
        new_context.second = line;
    }
    // Third line. Find the badge and sum the priority
    else {
        // Three lines of the group
        let first = &new_context.first;
        let second = &new_context.second;
        let third = line;

        // Search the common item
        let mut common_item = ' ';
        for item in first.chars() {
            if second.contains(item) && third.contains(item) {
                common_item = item;
            }
        }

        // Sum the priority
        let priority = compute_priority(common_item);
        let new_sum = context.sum + priority;
        new_context = Context {
            sum: new_sum,
            first: "".to_string(),
            second: "".to_string(),
        };
    }

    return new_context;
}

fn process_input(filename: &str) {
    // File Processing
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // Data management
    let mut context = Context {
        sum: 0,
        first: "".to_string(),
        second: "".to_string(),
    };

    for (_index, file_line) in reader.lines().enumerate() {
        let line = file_line.unwrap();
        context = process_line(line, context);
    }

    println!("Sum is {}", context.sum);
}

fn main() {
    // Read the input file
    let filename = "data/input.txt";
    process_input(filename);
}
