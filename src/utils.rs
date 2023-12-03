use regex::Regex;
use core::fmt;
use int_enum::IntEnum;

pub const DIGITS_PATTERN: &str = r"1|2|3|4|5|6|7|8|9";
pub const SPELLED_NUMBERS_PATTERN: &str = r"one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9";
pub const REVERSE_SPELLED_NUMBERS_PATTERN: &str = r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|1|2|3|4|5|6|7|8|9";

#[allow(dead_code)]
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum Day {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Eleven = 11,
    Twelve = 12,
    Thirteen = 13,
    Fourteen = 14,
    Fifteen = 15,
    Sixteen = 16,
    Seventeen = 17,
    Eighteen = 18,
    Nineteen = 19,
    Twenty = 20,
    TwentyOne = 21,
    TwentyTwo = 22,
    TwentyThree = 23,
    TwentyFour = 24,
    TwentyFive = 25,
}

#[allow(dead_code)]
#[repr(i32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum Part {
    One = 1,
    Two = 2,
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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

pub fn read_input(day: Day) -> String {
    let days_text: &str = &day.to_string().to_lowercase();
    let filepath = format!("./input/day_{}/input.txt", days_text);
    return read_file(&filepath); }

#[allow(dead_code)]
pub fn read_test_input(day: Day, part: Part) -> String {
    let days_text: &str = &day.to_string().to_lowercase();
    let filepath = format!("./input/day_{}/part_{}_test.txt", days_text, part.int_value());
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

pub fn print_day(day: Day, part_one: i32, part_two: i32) {
    println!("-------------------------");
    println!("#### Day {} ####", day.int_value());
    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}
