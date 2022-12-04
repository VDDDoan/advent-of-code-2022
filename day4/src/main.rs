use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let mut res = 0;
    let mut first_hashset: HashSet<u32> = HashSet::new();
    let mut second_hashset: HashSet<u32> = HashSet::new();
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let number_groups: Vec<&str> = ip.split(|c| c == ',' || c == '-').collect();
                for i in number_groups[0].parse::<u32>().unwrap()
                    ..number_groups[1].parse::<u32>().unwrap() + 1
                {
                    first_hashset.insert(i);
                }
                for i in number_groups[2].parse::<u32>().unwrap()
                    ..number_groups[3].parse::<u32>().unwrap() + 1
                {
                    second_hashset.insert(i);
                }
                if !first_hashset.is_disjoint(&second_hashset) {
                    res += 1;
                }
                first_hashset.clear();
                second_hashset.clear();
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
