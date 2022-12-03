use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut max: [u32; 3] = [0,0,0];
    let mut current: u32 = 0;
    let lines = read_lines("./in.txt").unwrap();
    
    for line in lines {
        current = match line.parse::<u32>() {
            Ok(num) => num + current,
            _ => {
                if current > max[0] {
                    max[0] = current;
                    for i in 1..max.len(){
                        if max[i-1] > max[i] {
                            current = max[i];
                            max[i] = max[i-1];
                            max[i-1] = current;
                        }
                    }
                }
                0
            },
        };
    }

    let sum: u32 = max.iter().sum();

    println!("{}",&sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>>{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().map(|l| l.unwrap()).collect())
}