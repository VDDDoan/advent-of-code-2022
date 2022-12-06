use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let mut res = String::new();
    let mut crates: Vec<Vec<char>> = Vec::from([
        Vec::from(['F', 'H', 'B', 'V', 'R', 'Q', 'D', 'P']),
        Vec::from(['L', 'D', 'Z', 'Q', 'W', 'V']),
        Vec::from(['H', 'L', 'Z', 'Q', 'G', 'R', 'F', 'C']),
        Vec::from(['R', 'D', 'H', 'F', 'J', 'V', 'B']),
        Vec::from(['Z', 'W', 'L', 'C']),
        Vec::from(['J', 'R', 'P', 'N', 'T', 'G', 'V', 'M']),
        Vec::from(['J', 'R', 'L', 'V', 'M', 'B', 'S']),
        Vec::from(['D', 'P', 'J']),
        Vec::from(['D', 'C', 'N', 'W', 'V']),
    ]);
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let instruction: Vec<&str> = ip.split(' ').collect();
                let mut movement: Vec<usize> = Vec::new();
                let mut temporary: Vec<char> = Vec::new();
                for i in 1..instruction.len() {
                    if let Ok(value) = instruction[i].parse::<usize>() {
                        movement.push(value);
                    }
                }
                for _ in 0..movement[0] {
                    if let Some(n) = crates[movement[1] - 1].pop() {
                        temporary.push(n);
                    }
                }
                for _ in 0..temporary.len() {
                    if let Some(n) = temporary.pop() {
                        crates[movement[2] - 1].push(n);
                    }
                }
            }
        }
        for i in 0..crates.len() {
            let len = crates[i].len();
            if len > 0 {
                res.push(crates[i][len - 1]);
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
