use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> io::Result<()> {
    let input_path = Path::new("input.txt");

    let lines: Vec<String> = io::BufReader::new(File::open(input_path)?)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    match part_one(&lines) {
        Ok(v) => println!("Part 1: {}", v),
        Err(e) => println!("{}", e),
    }

    match part_two(&lines) {
        Ok(v) => println!("Part 2: {}", v),
        Err(e) => println!("{}", e),
    }
    Ok(())
}
fn part_one(input: &[String]) -> Result<i32, Box<dyn Error>> {
    let mut safe_reports = 0;

    for line in input {
        let curr: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Not a number"))
            .collect();

        if test_line(&curr) {
            safe_reports += 1;
        }
    }

    Ok(safe_reports)
}

fn part_two(input: &[String]) -> Result<i32, Box<dyn Error>> {
    let mut safe_reports = 0;

    for line in input {
        let curr: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Not a number"))
            .collect();

        if test_line(&curr) {
            safe_reports += 1;
        } else {
            let mut test_vecs: Vec<Vec<i32>> = vec![];
            
            for i in 0..curr.len() {
                let mut test_vec: Vec<i32> = curr.clone();
                test_vec.remove(i);
                test_vecs.push(test_vec);
            }
            
            for vec in &test_vecs {
                if test_line(vec) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }

    Ok(safe_reports)
}

fn test_line(curr: &[i32]) -> bool {
    let mut increasing = false;
    let mut decreasing = false;

    for i in 1..curr.len() {
        let diff = curr[i] - curr[i - 1];

        if diff == 0 {
            return false;
        }

        if diff > 0 {
            increasing = true;
        } else {
            decreasing = true;
        }

        if !(1..=3).contains(&diff.abs()) {
            return false;
        }
    }

    if decreasing == increasing {
        return false;
    }
    true
}