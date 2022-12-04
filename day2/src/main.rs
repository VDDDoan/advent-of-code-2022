use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let mut res = 0;
    let game_hashmap: HashMap<char, HashMap<char, i32>> = HashMap::from([
        ('X', HashMap::from([('A', 3), ('B', 1), ('C', 2)])),
        ('Y', HashMap::from([('A', 4), ('B', 5), ('C', 6)])),
        ('Z', HashMap::from([('A', 8), ('B', 9), ('C', 7)])),
    ]);
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let char_vec: Vec<char> = ip.chars().collect();
                res += game_hashmap
                    .get(&char_vec[2])
                    .unwrap()
                    .get(&char_vec[0])
                    .unwrap();
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
