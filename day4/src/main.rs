use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let reader = io::BufReader::new(file);

    // Initialize a 2D vector of chars
    let mut matrix: Vec<Vec<char>> = Vec::new();

    // Read lines from the file
    for line in reader.lines() {
        let line = line.expect("readline"); // Handle potential I/O errors
        // Convert the line into a Vec<char>
        let row: Vec<char> = line.chars().collect();
        matrix.push(row); // Add the row to the 2D vector
    }

    let result = part1(&matrix);
    println!("Part 1: {}", result);
}

fn part1(grid: &[Vec<char>]) -> usize {
    let word = "XMAS";

    let directions = [
        (0, 1),  // Right
        (0, -1), // Left
        (1, 0),  // Down
        (-1, 0), // Up
        (1, 1),  // Down-right
        (1, -1), // Down-left
        (-1, 1), // Up-right
        (-1, -1) // Up-left
    ];

    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for row in 0..rows {
        for col in 0..cols {
            for &(dr, dc) in &directions {
                let mut found = true;
                for k in 0..word_len {
                    let nr = row as isize + k as isize * dr;
                    let nc = col as isize + k as isize * dc;
                    if nr < 0 || nc < 0 || nr >= rows as isize || nc >= cols as isize {
                        found = false;
                        break;
                    }
                    if grid[nr as usize][nc as usize] != word_chars[k] {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }
    count
}
