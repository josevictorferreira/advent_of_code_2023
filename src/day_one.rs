pub fn main() {
    let input_file_contents = read_input_file("./input/day_one.txt");
    let sum = sum_calibration_numbers(input_file_contents);
    println!("sum: {}", sum);
}

fn get_text_first_digit(input: &str) -> i32 {
    for char in input.chars() {
        if char.is_digit(10) {
            return char.to_string().as_str().parse().unwrap();
        }
    }
    panic!("No digit found in input: {}", input);
}

fn get_text_last_digit(input: &str) -> i32 {
    for char in input.chars().rev() {
        if char.is_digit(10) {
            return char.to_string().as_str().parse().unwrap();
        }
    }
    panic!("No digit found in input: {}", input);
}

fn get_calibration_number_from_line(input: &str) -> i32 {
    let first_digit: i32 = get_text_first_digit(input);
    let last_digit: i32 = get_text_last_digit(input);
    let calibration_digit: i32 = format!("{}{}", first_digit, last_digit).parse().unwrap();
    return calibration_digit;
}

fn read_input_file(filepath: &str) -> String {
    let file_contents =
        std::fs::read_to_string(filepath).expect("Unable to read file");
    return file_contents;
}

fn sum_calibration_numbers(text: String) -> i32 {
    let mut sum: i32 = 0;
    for line in text.lines() {
        let calibration_number = get_calibration_number_from_line(line);
        sum += calibration_number;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_digit() {
        assert_eq!(get_text_first_digit("1abc2"), 1);
    }
    #[test]
    #[should_panic]
    fn test_get_first_digit_panic() {
        get_text_first_digit("abc");
    }

    #[test]
    #[should_panic]
    fn test_get_last_digit_panic() {
        get_text_last_digit("abc");
    }

    #[test]
    #[should_panic]
    fn test_get_calibration_number_from_line_panic() {
        get_calibration_number_from_line("abc");
    }

    #[test]
    fn test_get_last_digit() {
        assert_eq!(get_text_last_digit("1abc2"), 2);
    }

    #[test]
    fn test_get_calibration_number_from_line() {
        assert_eq!(get_calibration_number_from_line("1abc2"), 12);
    }

    #[test]
    fn test_get_calibration_number_from_line_2() {
        assert_eq!(get_calibration_number_from_line("treb7uchet"), 77);
    }

    #[test]
    fn test_sum_calibration_numbers() {
        let test_file_contents = read_input_file("./input/day_one_test.txt");
        assert_eq!(sum_calibration_numbers(test_file_contents), 142);
    }
}
