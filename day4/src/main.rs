use std::{
    error::Error,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let input_path = Path::new("day4/input.txt");

    match part_one(input_path) {
        Ok(v) => println!("Part 1: {}", v),
        Err(e) => eprintln!("{}", e),
    }

    match part_two(input_path) {
        Ok(v) => println!("Part 2: {}", v),
        Err(e) => eprintln!("{}", e),
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
    DownRight,
    DownLeft,
    UpRight,
    UpLeft,
}

impl Direction {
    fn delta(self) -> (isize, isize) {
        match self {
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
            Direction::Down => (1, 0),
            Direction::Up => (-1, 0),
            Direction::DownRight => (1, 1),
            Direction::DownLeft => (1, -1),
            Direction::UpRight => (-1, 1),
            Direction::UpLeft => (-1, -1),
        }
    }

    fn all() -> &'static [Self] {
        &[
            Direction::Right,
            Direction::Left,
            Direction::Down,
            Direction::Up,
            Direction::DownRight,
            Direction::DownLeft,
            Direction::UpRight,
            Direction::UpLeft,
        ]
    }
}

fn part_one(input_path: &Path) -> Result<usize, Box<dyn Error>> {
    let grid = new_grid(input_path)?;
    let target = "XMAS";
    let mut total = 0;

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            for &direction in Direction::all() {
                let (dy, dx) = direction.delta();
                let mut matched = true;

                for i in 0..4 {
                    let new_row = row as isize + i * dy;
                    let new_col = col as isize + i * dx;

                    if new_row < 0
                        || new_col < 0
                        || new_row >= grid.len() as isize
                        || new_col >= grid[row].len() as isize
                        || grid[new_row as usize][new_col as usize]
                            != target.chars().nth(i as usize).unwrap()
                    {
                        matched = false;
                        break;
                    }
                }

                if matched {
                    total += 1;
                }
            }
        }
    }
    Ok(total)
}

fn part_two(input_path: &Path) -> Result<usize, Box<dyn Error>> {
    let grid = new_grid(input_path)?;
    let mut total = 0;

    for row in 1..grid.len() - 1 {
        for col in 1..grid[row].len() - 1 {

            if grid[row][col] != 'A' {continue;}
            
            // Up - Left
            match grid[row - 1][col - 1] {
                'A' | 'X' => continue,
                'S' => {if grid[row + 1][col + 1] != 'M' {continue}},
                'M' => {if grid[row + 1][col + 1] != 'S' {continue}},
                char => return Err(Box::<dyn Error>::from(format!("Bad data {char}"))),
            }


            // Up - Right
            match grid[row - 1][col + 1] {
                'A' | 'X' => continue,
                'S' => {if grid[row + 1][col - 1] != 'M' {continue}},
                'M' => {if grid[row + 1][col - 1] != 'S' {continue}},
                char => return Err(Box::<dyn Error>::from(format!("Bad data {char}"))),
            }

            total += 1
        }
    }

    Ok(total)
}

fn new_grid(input_path: &Path) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let grid = io::BufReader::new(File::open(input_path)?)
        .lines()
        .into_iter()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    Ok(grid)
}

#[cfg(test)]
mod day4_tests {
    use super::*;
    #[test]
    fn ex_part_1() {
        let input_path = Path::new("example.txt");
        let result = part_one(input_path).unwrap();
        assert_eq!(result, 18);
    }

    #[test]
    fn ex_part_2() {
        let input_path = Path::new("example.txt");
        let result = part_two(input_path).unwrap();
        assert_eq!(result, 9);
    }
}
