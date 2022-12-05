use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

const STACKS_COUNT: usize = 9;
const STACK: Vec<char> = Vec::new();

struct Context {
    move_part: bool,
    stacks: [Vec<char>; STACKS_COUNT],
}

fn apply_move(line: String, context: Context) -> Context {
    // Copy the context
    let mut new_context = Context {
        move_part: true,
        stacks: [STACK; STACKS_COUNT],
    };
    for i in 0..STACKS_COUNT {
        new_context.stacks[i] = context.stacks[i].to_vec();
    }

    let re = Regex::new("move (\\d+) from (\\d) to (\\d)").unwrap();
    for cap in re.captures_iter(line.as_str()) {
        // Extract the arguments of the move
        let nb_moves = cap[1].parse::<usize>().unwrap();
        let move_from = cap[2].parse::<usize>().unwrap();
        let move_to = cap[3].parse::<usize>().unwrap();

        println!("Move {} times from {} to {}", nb_moves, move_from, move_to);

        // Apply them
        for _move_index in 0..nb_moves {
            let poped = new_context.stacks[move_from - 1].pop().unwrap();
            new_context.stacks[move_to - 1].push(poped);
        }
    }
    return new_context;
}

fn build_state(line: String, context: Context) -> Context {
    // Prepare a new context
    let mut new_context = Context {
        move_part: false,
        stacks: [STACK; STACKS_COUNT],
    };

    for i in 0..STACKS_COUNT {
        // Copy the current stack
        new_context.stacks[i] = context.stacks[i].to_vec();

        // Extract the value of the stack
        let stack_delim = line.substring(i * 4, i * 4 + 1);
        let stack_value = line.substring(i * 4 + 1, i * 4 + 2);
        if stack_delim.eq("[") && !stack_value.eq(" ") {
            // Push the value on the appropriate stack (front push)
            let stack_char = stack_value.chars().next().unwrap();
            new_context.stacks[i].insert(0, stack_char);
        }
    }
    return new_context;
}

fn process_line(line: String, context: Context) -> Context {
    let new_context;
    if line.is_empty() {
        new_context = Context {
            move_part: true,
            stacks: context.stacks,
        };
    } else if context.move_part {
        new_context = apply_move(line, context)
    } else {
        new_context = build_state(line, context)
    }
    return new_context;
}

fn process_input(filename: &str) {
    // File Processing
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    
    // Data management
    let mut context = Context {
        move_part: false,
        stacks: [STACK; STACKS_COUNT],
    };

    for (_index, file_line) in reader.lines().enumerate() {
        let line = file_line.unwrap();
        context = process_line(line, context);
    }

    let mut tops = String::from("");
    for i in 0..STACKS_COUNT {
        tops.push_str(context.stacks[i].pop().unwrap().to_string().as_str());
    }

    println!("Stack tops are {}", tops);
}

fn main() {
    // Read the input file
    let filename = "data/input.txt";
    process_input(filename);
}
