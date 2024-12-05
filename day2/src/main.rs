use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input_path = Path::new("input.txt");

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
        let mut increasing = false;
        let mut decreasing = false;

        for i in 1..curr.len() {
            let diff = curr[i] - curr[i - 1];

            if diff == 0 {
                safe = false;
                break;
            }
            
            if diff > 0 {
                increasing = true;
            } else {
                decreasing = true;
            }

            if !(1..=3).contains(&diff.abs()) {
                safe = false;
                break;
            }
        }
        
        if decreasing == increasing {
            safe = false;
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