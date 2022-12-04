use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let mut calorie_sum: Vec<i32> = vec![];
    let mut current_sum = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                match ip.parse::<i32>() {
                    Ok(n) => {
                        current_sum += n;
                    }
                    Err(_) => {
                        calorie_sum.push(current_sum);
                        current_sum = 0;
                    }
                }
            }
        }
    }

    calorie_sum.sort_by(|a, b| b.cmp(a));
    let mut max = 0;
    for i in 0..3 {
        max += calorie_sum[i];
    }
    println!("{}", max);
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
