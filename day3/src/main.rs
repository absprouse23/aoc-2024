use regex::Regex;

fn main() {
    let data = std::fs::read_to_string("input.txt").expect("failed to read file");
    println!("Part 1: {}", part_one(&data));
    println!("Part 2: {}", part_two(&data));
}

fn part_one(data: &String) -> i32 {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let valid_muls: Vec<&str> = re
        .find_iter(&data)
        .map(|cap| cap.as_str())
        .collect();

    let mut result = 0;
    for instr in valid_muls {
        result += parse_mul(instr.parse().unwrap());
    }
    result
}

fn part_two(data: &String) -> i32 {
    let mut result = 0;
    let re = Regex::new(r"(don't\(\))|(do\(\))|(mul\([0-9]+,[0-9]+\))").unwrap();
    let instructions: Vec<&str> = re
        .find_iter(&data)
        .map(|cap| cap.as_str())
        .collect();

    let mut ignore_mul = false;

    for instr in instructions {
        match instr {
            "do()" => ignore_mul = false,
            "don't()" => ignore_mul = true,
            _ => match ignore_mul{
                true => {},
                false => {result += parse_mul(instr.parse().unwrap())}
            }
        }
    }

    result
}

fn parse_mul(instr: String) -> i32 {
    let instr = instr
        .strip_prefix("mul(")
        .unwrap()
        .strip_suffix(")")
        .unwrap();

    let nums = instr
        .split(",")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.parse::<i32>())
        .collect::<Result<Vec<i32>, std::num::ParseIntError>>()
        .expect("failed to parse nums");

    nums[0] * nums[1]
}
