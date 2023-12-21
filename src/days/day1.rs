use aho_corasick::AhoCorasick;


fn join_numbers(a: u8, b: u8) -> u32 {
    let joined = format!("{}{}", a, b);
    joined.parse::<u32>().unwrap()
}

pub fn part1() {
    let mut count = 0;
    let characters = include_str!("/home/dru/Rust_projects/aoc2023/src/data/data1.txt");
    for ch in characters.lines() {
        let digits = ch.chars()
            .filter(|x| x.is_ascii_digit())
            .map(|x| x as u8 - b'0')
            .collect::<Vec<_>>();
        match digits.len() {
            1 => {
                let temp_digit = join_numbers(digits[0], digits[0]);
                count += temp_digit;
            }
            _ => {
                let temp_digit = join_numbers(digits[0], digits.last().copied().unwrap());
                count += temp_digit;
            }
        }
    }
    println!("{}", count);
}

// convert number match to digit
fn convert_match_to_number(pattern: &str) -> u8 {
    match pattern {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0
    }
}

pub fn part2() {
    let patterns = [
                    "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6",
                                 "seven", "7", "eight", "8", "nine", "9"];
    let puzzle_input = include_str!("/home/dru/Rust_projects/aoc2023/src/data/data1.txt");
    let ac = AhoCorasick::new(patterns).unwrap(); // algorithm used to search for patterns

    let mut count = 0;
    for lines in puzzle_input.lines() {
        let mut matched_characters: Vec<u8> = vec![];
        // iterate over each line of input
        for mat in ac.find_overlapping_iter(lines) {
            let pattern_id = convert_match_to_number(patterns[mat.pattern().as_usize()]);
            matched_characters.push(pattern_id);
        }
        // println!("{:?}", matched_characters);
        match matched_characters.len() {
            1 => {
                let temp_digit = join_numbers(matched_characters[0], matched_characters[0]);
                count += temp_digit;
            },
            _ => {
                let temp_digit = join_numbers(matched_characters[0], matched_characters.last().copied().unwrap());
                count += temp_digit;
            }
        }
    }
    println!("{:?}", count);
}