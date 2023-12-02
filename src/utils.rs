#![allow(dead_code)]
pub fn read_input(input_day_name: &str) -> String {
    let filepath = format!("./input/{}.txt", input_day_name);
    let file_contents =
    std::fs::read_to_string(filepath).expect("Unable to read file");
    return file_contents;
}
