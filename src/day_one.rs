#[path = "./utils.rs"]
mod utils;

use utils::Day;
use utils::DIGITS_PATTERN;
use utils::REVERSE_SPELLED_NUMBERS_PATTERN;
use utils::SPELLED_NUMBERS_PATTERN;

pub fn main() {
    let input = utils::read_input(Day::One);
    let sum_part_one = sum_calibration_digits(input.clone());
    let sum_part_two = sum_calibration_spelled_numbers(input);
    utils::print_day(Day::One, sum_part_one, sum_part_two);
}

fn get_calibration_digit_from_line(input: &str) -> i32 {
    let first_digit: i32 = utils::get_text_first_ocurrence(input, DIGITS_PATTERN);
    let last_digit: i32 = utils::get_text_last_ocurrence(input, DIGITS_PATTERN);
    let calibration_digit: i32 = format!("{}{}", first_digit, last_digit).parse().unwrap();
    return calibration_digit;
}

fn get_calibration_spelled_number_from_line(input: &str) -> i32 {
    let first_digit: i32 = utils::get_text_first_ocurrence(input, SPELLED_NUMBERS_PATTERN);
    let last_digit: i32 = utils::get_text_last_ocurrence(input, REVERSE_SPELLED_NUMBERS_PATTERN);
    let calibration_digit: i32 = format!("{}{}", first_digit, last_digit).parse().unwrap();
    return calibration_digit;
}

fn sum_calibration_digits(text: String) -> i32 {
    let mut sum: i32 = 0;
    for line in text.lines() {
        let calibration_number = get_calibration_digit_from_line(line);
        sum += calibration_number;
    }
    return sum;
}

fn sum_calibration_spelled_numbers(text: String) -> i32 {
    let mut sum: i32 = 0;
    for line in text.lines() {
        let calibration_number = get_calibration_spelled_number_from_line(line);
        sum += calibration_number;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::Part;

    #[test]
    fn test_get_calibration_digit_from_line() {
        assert_eq!(get_calibration_digit_from_line("1abc2"), 12);
    }

    #[test]
    fn test_get_calibration_digit_from_line_2() {
        assert_eq!(get_calibration_digit_from_line("treb7uchet"), 77);
    }

    #[test]
    fn test_get_calibration_spelled_number_from_line() {
        assert_eq!(get_calibration_spelled_number_from_line("two1nine"), 29);
    }

    #[test]
    fn test_get_calibration_spelled_number_from_line_2() {
        assert_eq!(get_calibration_spelled_number_from_line("zoneight234"), 14);
    }

    #[test]
    fn test_get_calibration_spelled_number_from_line_3() {
        assert_eq!(get_calibration_spelled_number_from_line("eighthree"), 83);
    }

    #[test]
    fn test_get_calibration_spelled_number_from_line_4() {
        assert_eq!(get_calibration_spelled_number_from_line("sevenine"), 79);
    }

    #[test]
    fn test_sum_calibration_digits() {
        let test_file_contents = utils::read_test_input(Day::One, Part::One);
        assert_eq!(sum_calibration_digits(test_file_contents), 142);
    }

    #[test]
    fn test_sum_calibration_spelled_numbers() {
        let test_file_contents = utils::read_test_input(Day::One, Part::Two);
        assert_eq!(sum_calibration_spelled_numbers(test_file_contents), 281);
    }
}
