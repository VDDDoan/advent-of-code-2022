use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let mut res = 0;
    let mut char_hashmap: HashMap<char, u32> = HashMap::new();
    let mut char_hashset: HashSet<char> = HashSet::new();
    let mut line_count = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                line_count += 1;
                for ch in ip.chars() {
                    char_hashset.insert(ch);
                }
                for &ch in char_hashset.iter() {
                    char_hashmap
                        .entry(ch)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);
                }
                char_hashset.clear();
                if line_count == 3 {
                    line_count = 0;
                    for (key, val) in char_hashmap.iter() {
                        if *val == 3 && key.is_uppercase() {
                            res += *key as u32 - 38;
                        } else if *val == 3 && key.is_lowercase() {
                            res += *key as u32 - 96;
                        }
                    }
                    char_hashmap.clear();
                }
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
