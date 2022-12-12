use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct FileType {
    size: usize,
}
#[derive(Debug)]
struct Directory {
    children: Vec<usize>,
}
#[derive(Debug)]
enum NodeType {
    Dir(Directory),
    Fil(FileType),
}
#[derive(Debug)]
struct Node {
    parent_uid: usize,
    name: String,
    node: NodeType,
}

fn calculate_size(file_system: &Vec<Node>, idx: usize) -> usize {
    match &file_system[idx].node {
        NodeType::Dir(dir) => {
            let mut total = 0;
            for i in 0..dir.children.len() {
                total += calculate_size(&file_system, dir.children[i]);
            }
            return total;
        }
        NodeType::Fil(fil) => {
            return fil.size;
        }
    }
}
fn main() {
    // File hosts must exist in current path before this produces output
    let mut file_system: Vec<Node> = Vec::new();
    file_system.push(Node {
        name: "/".to_string(),
        parent_uid: 0,
        node: NodeType::Dir(Directory {
            children: Vec::new(),
        }),
    });
    let mut current_dir_idx = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.starts_with("$ ls") {
                    continue;
                }
                let parsed_string: Vec<&str> = ip.split_whitespace().collect();
                if parsed_string.len() == 3 && parsed_string[2] == ".." {
                    current_dir_idx = file_system[current_dir_idx].parent_uid;
                } else if parsed_string.len() == 3 {
                    if let NodeType::Dir(directory) = &file_system[current_dir_idx].node {
                        for i in 0..directory.children.len() {
                            if let NodeType::Dir(_) = file_system[directory.children[i]].node {
                                if file_system[directory.children[i]].name
                                    == parsed_string[2].to_string()
                                {
                                    current_dir_idx = directory.children[i];
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    if parsed_string[0] == "dir" {
                        file_system.push(Node {
                            name: parsed_string[1].to_string(),
                            parent_uid: current_dir_idx,
                            node: NodeType::Dir(Directory {
                                children: Vec::new(),
                            }),
                        });
                    } else {
                        file_system.push(Node {
                            name: parsed_string[1].to_string(),
                            parent_uid: current_dir_idx,
                            node: NodeType::Fil(FileType {
                                size: parsed_string[0].parse::<usize>().unwrap(),
                            }),
                        });
                    }
                    let mut new_vec: Vec<usize> = Vec::new();
                    if let NodeType::Dir(directory) = &file_system[current_dir_idx].node {
                        new_vec = directory.children.clone();
                        new_vec.push(file_system.len() - 1);
                    }
                    file_system[current_dir_idx].node =
                        NodeType::Dir(Directory { children: new_vec });
                }
            }
        }
    }
    let mut res = calculate_size(&file_system, 0);
    let free_space = 70000000 - res;
    let size_to_free = 30000000 - free_space;
    // println!("{:?}", file_system);
    for i in 1..file_system.len() {
        if let NodeType::Dir(_) = &file_system[i].node {
            let current_dir_size = calculate_size(&file_system, i);
            if current_dir_size > size_to_free && current_dir_size < res {
                res = current_dir_size;
            }
        }
    }
    println!("{}", res);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
