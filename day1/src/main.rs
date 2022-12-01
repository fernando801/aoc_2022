use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut max: u32 = 0;
    let mut current: u32 = 0;
    let lines = read_lines("./in.txt").unwrap();
    for line in lines {
        current = match line.parse::<u32>() {
            Ok(num) => num + current,
            _ => 0,
        };
        if current > max {
            max = current;
        }
    }
    println!("{}",&max);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>>{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().map(|l| l.unwrap()).collect())
}