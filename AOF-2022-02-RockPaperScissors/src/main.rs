use std::fs::File;
use std::io::{BufRead, BufReader};

fn process_line(line: String, score: i32) -> i32 {
    let mut score_increment = 0;
    // Parse the line as a command
    let mut line_tokens = line.split(" ");
    let opponent_choice = line_tokens.next().unwrap().chars().next().unwrap();
    let my_choice = line_tokens.next().unwrap().chars().next().unwrap();
    // Decode choice
    let opponent_value = opponent_choice as i32 - 'A' as i32;
    let my_value = my_choice as i32 - 'X' as i32;
    // Fight values
    let delta = my_value - opponent_value;
    match delta {
        0 => score_increment = score_increment + 3,
        1 | -2 => score_increment = score_increment + 6,
        _ => score_increment = score_increment + 0,
    }
    // Choice bonus
    score_increment = score_increment + 1 + my_value;
    
    return score + score_increment;
}

fn process_input(filename: &str) {
    // File Processing
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // Data management
    let mut score: i32 = 0;

    for (_index, file_line) in reader.lines().enumerate() {
        let line = file_line.unwrap();
        score = process_line(line, score);
    }

    println!("Score is {}", score);
}

fn main() {
    // Read the input file
    let filename = "data/input.txt";
    process_input(filename);
}
