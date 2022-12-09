use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

struct Directory {
    id: i32,
    parent: i32,
    size: u32,
    files: HashMap<String, u32>,
    directories: HashMap<String, i32>,
}
struct DirectoryTree {
    current: i32,
    last_id: i32,
    directories: HashMap<i32, Directory>,
}
impl DirectoryTree {
    // Change the current directory to the parent
    fn cd_up(&mut self) {
        let current_dir = self.directories.get(&self.current);
        if current_dir.is_none() {
            self.current = 0;
        } else {
            self.current = current_dir.unwrap().parent;
        }
    }

    fn cd(&mut self, sub_dir_name: &String) {
        let current_dir = self.directories.get(&self.current).unwrap();
        let sub_dir_id = current_dir.directories.get(sub_dir_name).unwrap();
        self.current = *sub_dir_id;
    }

    fn create_sub_directory(&mut self, sub_dir_name: String) {
        let current_dir = self.directories.get_mut(&self.current).unwrap();
        self.last_id = self.last_id + 1;
        let sub_dir = Directory {
            id: self.last_id,
            parent: current_dir.id,
            size: 0,
            files: HashMap::new(),
            directories: HashMap::new(),
        };
        current_dir.directories.insert(sub_dir_name, self.last_id);
        self.directories.insert(self.last_id, sub_dir);
    }

    fn create_file(&mut self, name: String, size: u32) {
        // Add the file
        let current_dir = self.directories.get_mut(&self.current).unwrap();
        current_dir.files.insert(name, size);

        // Update the size of the directory and its parents
        current_dir.size = current_dir.size + size;
        let mut parent_id = current_dir.parent;
        while parent_id >= 0 {
            let parent = self.directories.get_mut(&parent_id).unwrap();
            parent.size = parent.size + size;
            parent_id = parent.parent;
        }
    }
}

fn process_line(line: String, tree: &mut DirectoryTree) {
    if line.starts_with("$ cd ..") {
        // Move to parent directory
        tree.cd_up();
    } else if line.starts_with("$ cd") {
        // Move the child directory
        let sub_dir_name = line.substring(5, line.len());
        tree.cd(&String::from(sub_dir_name));
    } else if line.starts_with("dir ") {
        // Add a directory
        let sub_dir_name = line.substring(4, line.len());
        tree.create_sub_directory(String::from(sub_dir_name));
    } else if line.ne("$ ls") {
        // Add a file
        let mut line_tokens = line.split(" ");
        let file_size: u32 = line_tokens.next().unwrap().parse::<u32>().unwrap();
        let file_name = line_tokens.next().unwrap();
        tree.create_file(String::from(file_name), file_size);
    }
}

fn compute_total_size(tree: &DirectoryTree) -> u32 {
    // Cumulated size
    let mut size = 0;
    for directory in tree.directories.values() {
        if directory.size <= 100_000 {
            size = size + directory.size;
        }
    }
    return size;
}

fn process_input(filename: &str) {
    // File Processing
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    // Data management
    let root_directory = Directory {
        id: 0,
        parent: -1,
        size: 0,
        directories: HashMap::new(),
        files: HashMap::new(),
    };
    let mut tree = DirectoryTree {
        current: 0,
        last_id: 0,
        directories: HashMap::new(),
    };
    tree.directories.insert(0, root_directory);

    for (_index, file_line) in reader.lines().enumerate() {
        let line = file_line.unwrap();
        process_line(line, &mut tree);
    }

    println!("Size of directories is {}", compute_total_size(&tree));
}

fn main() {
    // Read the input file
    let filename = "data/input.txt";
    process_input(filename);
}
