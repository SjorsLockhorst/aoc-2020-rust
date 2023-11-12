use aoc_2020_rust::util;

use advent_of_code::solve;

fn is_valid_pass(lower: u32, upper: u32, target_char: char, password: &str) -> bool {
    let mut found: u32 = 0;
    for char in password.to_string().chars() {
        if char == target_char {
            found += 1
        }
    }
    if found >= lower && found <= upper {
        return true
    }
    return false
}
fn is_valid_pass_day_2(first_position: usize, second_position: usize, target_char: char, password: &str) -> bool {
    let first_char = password.chars().nth(first_position - 1).unwrap();
    let second_char = password.chars().nth(second_position - 1).unwrap();
    let either: bool = first_char == target_char || second_char == target_char;
    let both: bool = first_char == target_char && second_char == target_char;
    either && !both
}
fn part_one(clean_input: &Vec<&str>) -> i32 {

    let mut output: Vec<bool> = Vec::new();

    for line in clean_input {
        let (raw_policy, password) = line.split_once(": ").unwrap();
        let (range, target_char_str) = raw_policy.split_once(" ").unwrap();
        let target_char = target_char_str.to_string().parse::<char>().unwrap();

        let (lower_str, upper_str) = range.split_once("-").unwrap();
        let lower = lower_str.to_string().parse::<u32>().unwrap();
        let upper = upper_str.to_string().parse::<u32>().unwrap();
        output.push(is_valid_pass(lower, upper, target_char, password));
    }
    let result: i32 = output.iter().map(|&x| if x {1} else {0}).sum();
    result

}

fn part_two(clean_input: &Vec<&str>) -> i32 {

    let mut output: Vec<bool> = Vec::new();
    for line in clean_input {

        let (raw_policy, password) = line.split_once(": ").unwrap();
        let (range, target_char_str) = raw_policy.split_once(" ").unwrap();
        let target_char = target_char_str.to_string().parse::<char>().unwrap();

        let (first_pos_str, second_pos_str) = range.split_once("-").unwrap();
        let first_position = first_pos_str.to_string().parse::<u32>().unwrap();
        let second_position = second_pos_str.to_string().parse::<u32>().unwrap();
            output.push(is_valid_pass_day_2(first_position.try_into().unwrap(), second_position.try_into().unwrap(), target_char, password));
    }
    let result: i32 = output.iter().map(|&x| if x {1} else {0}).sum();
    result
}


fn main() {
    let input = util::load_input(2).unwrap();
    let clean_input: Vec<&str> = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .collect();

    let res_part_one = part_one(&clean_input);
    assert_eq!(solve(2020, 2, 1, &input), Ok(res_part_one.to_string()));

    let res_part_two = part_two(&clean_input);
    assert_eq!(solve(2020, 2, 2, &input), Ok(res_part_two.to_string()));
}
