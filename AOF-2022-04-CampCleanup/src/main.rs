use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn process_line(line: String, sum: i32) -> i32 {
    let mut new_sum = sum;
    let re = Regex::new("(\\d+)-(\\d+),(\\d+)-(\\d+)").unwrap();
    for cap in re.captures_iter(line.as_str()) {
        // Pair 1 boundaries
        let min1 = cap[1].parse::<u32>().unwrap();
        let max1 = cap[2].parse::<u32>().unwrap();
        // Pair 2 boundaries
        let min2 = cap[3].parse::<u32>().unwrap();
        let max2 = cap[4].parse::<u32>().unwrap();

        if max1>=min2 && max2>=min1 {
            new_sum = new_sum + 1;
        }
    }
    return new_sum;
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
