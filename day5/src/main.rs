use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let input_path = Path::new("day5/input.txt");

    match part_one(input_path) {
        Ok(v) => println!("Part 1: {}", v),
        Err(e) => eprintln!("{}", e),
    }

    match part_two(input_path) {
        Ok(v) => println!("Part 2: {}", v),
        Err(e) => eprintln!("{}", e),
    }
}

fn part_one(input_path: &Path) -> Result<i32, Box<dyn Error>> {
    let (rules, updates) = parse_rules(input_path)?;
    println!("Rules: {:#?}", rules);
    println!("Updates: {:#?}", updates);
    todo!();
}

fn part_two(input_path: &Path) -> Result<i32, Box<dyn Error>> {
    todo!();
}

fn parse_rules(input_path: &Path) -> Result<(Vec<(u16, u16)>, Vec<Vec<u16>>), Box<dyn Error>> {
    let mut lines: Vec<String> = io::BufReader::new(File::open(input_path)?)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let mut rules: Vec<(u16, u16)> = Vec::new();
    let mut i =0;

    while i < lines.len() && !lines[i].trim().is_empty() {
        let mut parts = lines[i].split('|');
        let from = parts
            .next()
            .ok_or("Missing 'from' value")?
            .parse::<u16>()?;
        let to = parts
            .next()
            .ok_or("Missing 'to' value")?
            .parse::<u16>()?;
        rules.push((from, to));
        i += 1;
    }

    i += 1;

    let updates = lines[i..]
        .iter()
        .map(|update| {
            update
                .split(',')
                .map(|page| page.trim().parse::<u16>().unwrap())
                .collect()
        })
        .collect();

    Ok((rules, updates))
}
