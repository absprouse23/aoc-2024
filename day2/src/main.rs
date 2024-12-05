use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input_path = Path::new("example.txt");

    match part_one(input_path) {
        Ok(v) => println!("Part 1: {}", v),
        Err(e) => println!("{}", e),
    }
}
fn part_one(input_path: &Path) -> Result<i32, Box<dyn Error>> {
    let mut safe_reports = 0;
    let lines = io::BufReader::new(File::open(input_path)?)
        .lines()
        .map(|l| l.unwrap());

    for line in lines {
        let curr: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;
        
        let mut safe = true;
        let mut increasing: Option<bool> = None;
        
        for i in 0..curr.len() {
            if i == 0 {continue}
            let diff = curr[i] - curr[i - 1];
            
            if diff == 0 {
                safe = false;
                break;
            }

            if i == 1 {
                if diff > 0 {
                    increasing = Some(true);
                } else {
                    increasing = Some(false);
                }
            }
            
            if increasing.unwrap() && diff < 0 {
                safe = false; 
                break;
            }
            
            if !(1..=3).contains(&diff.abs()) {
                safe = false;
                break;
            }
        }
        
        if safe {
            println!("{} Safe", line);
            safe_reports += 1;
        } else { 
            println!("{} Unsafe", line);
        }
    }

    Ok(safe_reports)
}