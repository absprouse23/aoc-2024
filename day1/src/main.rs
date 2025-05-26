use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() {
    let input_path = Path::new("day1/input.txt");

    match part_one(input_path) {
        Ok(v) => println!("Part 1: {}", v),
        Err(e) => println!("{}", e),
    }

    match part_two(input_path) {
        Ok(v) => println!("Part 2: {}", v),
        Err(e) => println!("{}", e),
    }
}

fn part_one(input_path: &Path) -> Result<i32, Box<dyn Error>> {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let fp = File::open(input_path)?;
    let lines = io::BufReader::new(fp).lines().map(|l| l.unwrap());

    for line in lines {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();
        list1.push(split_line[0].parse::<i32>()?);
        list2.push(split_line[1].parse::<i32>()?);
    }

    list1.sort_unstable();
    list2.sort_unstable();

    let mut sum = 0;

    for (a, b) in list1.iter().zip(list2.iter()) {
        sum += (*a - *b).abs();
    }

    Ok(sum)
}

fn part_two(input_path: &Path) -> Result<i32, Box<dyn Error>> {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let fp = File::open(input_path)?;
    let lines = io::BufReader::new(fp).lines().map(|l| l.unwrap());

    for line in lines {
        let split_line = line.split_whitespace().collect::<Vec<&str>>();
        list1.push(split_line[0].parse::<i32>()?);
        list2.push(split_line[1].parse::<i32>()?);
    }

    let mut sim_score = 0;

    for num in list1.iter() {
        sim_score += num * list2.iter().filter(|&x| x == num).count() as i32;
    }

    Ok(sim_score)
}

#[cfg(test)]
mod day1_tests {
    use super::*;

    #[test]
    fn ex_part_1() {
        let input_path = Path::new("example.txt");
        let result = part_one(input_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 11)
    }

    #[test]
    fn ex_part_2() {
        let input_path = Path::new("example.txt");
        let result = part_two(input_path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 31)
    }
}
