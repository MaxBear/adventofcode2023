use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut _sum = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let v: Vec<&str> = ip
                    .split("")
                    .filter(|&element| element.len() > 0 && element.as_bytes()[0].is_ascii_digit())
                    .collect();

                let first = v[0].parse::<i32>().unwrap();
                let mut last = first;
                if v.len() > 1 {
                    last = v[v.len() - 1].parse::<i32>().unwrap();
                }
                _sum += first * 10 + last;
            }
        }
    }
    println!("{}", _sum)
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
