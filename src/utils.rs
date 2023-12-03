use regex::Regex;
use std::collections::HashMap;

pub const DAY_ONE: i32 = 1;

#[allow(dead_code)]
pub const PART_ONE: i32 = 1;
#[allow(dead_code)]
pub const PART_TWO: i32 = 2;

pub const DIGITS_PATTERN: &str = r"1|2|3|4|5|6|7|8|9";
pub const SPELLED_NUMBERS_PATTERN: &str = r"one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9";
pub const REVERSE_SPELLED_NUMBERS_PATTERN: &str = r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|1|2|3|4|5|6|7|8|9";

fn get_days_text() -> HashMap<i32, &'static str> {
    return HashMap::from([
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (21, "twenty_one"),
        (22, "twenty_two"),
        (23, "twenty_three"),
        (24, "twenty_four"),
        (25, "twenty_five"),
    ]);
}

fn word_to_number(word: &str) -> Option<i32> {
    match word {
        "one" | "1" => Some(1),
        "two" | "2" => Some(2),
        "three" | "3" => Some(3),
        "four" | "4" => Some(4),
        "five" | "5" => Some(5),
        "six" | "6" => Some(6),
        "seven" | "7" => Some(7),
        "eight" | "8" => Some(8),
        "nine" | "9" => Some(9),
        _ => None,
    }
}

fn word_reverse_to_number(word: &str) -> Option<i32> {
    match word {
        "eno" | "1" => Some(1),
        "owt" | "2" => Some(2),
        "eerht" | "3" => Some(3),
        "ruof" | "4" => Some(4),
        "evif" | "5" => Some(5),
        "xis" | "6" => Some(6),
        "neves" | "7" => Some(7),
        "thgie" | "8" => Some(8),
        "enin" | "9" => Some(9),
        _ => None,
    }
}

pub fn read_input(day_num: i32) -> String {
    let days_text: HashMap<i32, &str> = get_days_text();
    let input_day_name = days_text.get(&day_num).unwrap();
    let filepath = format!("./input/day_{}/input.txt", input_day_name);
    return read_file(&filepath);
}

#[allow(dead_code)]
pub fn read_test_input(day_num: i32, part_num: i32) -> String {
    let days_text: HashMap<i32, &str> = get_days_text();
    let input_day_name = days_text.get(&day_num).unwrap();
    let filepath = format!("./input/day_{}/part_{}_test.txt", input_day_name, part_num);
    return read_file(&filepath);
}

fn read_file(filepath: &str) -> String {
    let file_contents = std::fs::read_to_string(filepath).expect("Unable to read file");
    return file_contents;
}

pub fn get_text_first_ocurrence(input: &str, pattern: &str) -> i32 {
    let re = Regex::new(&pattern).unwrap();
    let first_match = re.find(input).unwrap().as_str();
    return match word_to_number(first_match) {
        Some(number) => number,
        None => panic!("Could not convert {} to number", first_match),
    };
}

pub fn get_text_last_ocurrence(input: &str, reverse_pattern: &str) -> i32 {
    let reverse_input = input.chars().rev().collect::<String>();
    let re = Regex::new(&reverse_pattern).unwrap();
    let first_match = re.find(&reverse_input).unwrap().as_str();
    return match word_reverse_to_number(first_match) {
        Some(number) => number,
        None => panic!("Could not convert {} to number", first_match),
    };
}

pub fn print_day(day_num: i32, part_one: i32, part_two: i32) {
    println!("-------------------------");
    println!("#### Day {} ####", day_num);
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}
