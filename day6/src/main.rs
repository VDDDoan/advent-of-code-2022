use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let mut char_hashmap: HashMap<char, usize> = HashMap::new();
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let char_vector: Vec<char> = ip.chars().collect();
                for i in 0..char_vector.len() {
                    if let Some(x) = char_hashmap.get(&char_vector[i]) {
                        let y = x.clone();
                        char_hashmap.retain(|_, v| *v > y);
                    }
                    char_hashmap.insert(char_vector[i], i);
                    if char_hashmap.len() == 14 {
                        println!("{}", i + 1);
                        break;
                    }
                }
            }
        }
    }
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
