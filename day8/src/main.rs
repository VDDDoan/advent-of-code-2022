use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut tree_map: Vec<Vec<u32>> = Vec::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let number_char: Vec<char> = ip.chars().collect();
                let mut row: Vec<u32> = Vec::new();
                for i in 0..number_char.len() {
                    row.push(number_char[i].to_digit(10).unwrap());
                }
                tree_map.push(row);
            }
        }
    }

    let index_row = vec![0; tree_map[0].len()];

    let mut index_map: Vec<Vec<u32>> = Vec::new();

    for _ in 0..tree_map.len() {
        index_map.push(index_row.clone());
    }

    for (y, row) in tree_map.iter().enumerate() {
        for (x, el) in row.iter().enumerate() {
            let mut north = y as i32 - 1;
            let mut west = x as i32 - 1;
            let mut east = x + 1;
            let mut south = y + 1;
            let mut count_vec: Vec<u32> = Vec::new();
            let mut count = 0;
            while north >= 0 && tree_map[north as usize][x] < *el {
                count += 1;
                north -= 1;
            }
            if north >= 0 {
                count += 1;
            }
            if count > 0 {
                count_vec.push(count);
            }
            count = 0;
            while south < tree_map.len() && tree_map[south][x] < *el {
                count += 1;
                south += 1;
            }
            if south < tree_map.len() {
                count += 1;
            }
            if count > 0 {
                count_vec.push(count);
            }
            count = 0;
            while east < row.len() && tree_map[y][east] < *el {
                count += 1;
                east += 1;
            }
            if east < row.len() {
                count += 1;
            }
            if count > 0 {
                count_vec.push(count);
            }
            count = 0;
            while west >= 0 && tree_map[y][west as usize] < *el {
                count += 1;
                west -= 1;
            }
            if west >= 0 {
                count += 1;
            }
            if count > 0 {
                count_vec.push(count);
            }
            let mut points = 1;
            for i in 0..count_vec.len() {
                points *= count_vec[i];
            }
            index_map[y][x] = points as u32;
        }
    }

    let mut scenic_points = 0;

    for row in index_map.iter() {
        for &el in row.iter() {
            if el > scenic_points {
                scenic_points = el;
            }
        }
    }

    println!("{}", scenic_points);
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
