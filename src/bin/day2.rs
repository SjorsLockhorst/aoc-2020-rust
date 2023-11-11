mod util;

use advent_of_code::solve;

fn is_valid_pass(lower: u32, upper: u32, target_char: char, password: &str) -> u32 {
    let mut found: u32 = 0;
    print!("{}\n", found);
    for char in password.to_string().chars() {
        if char == target_char {
            found += 1
        }
    }
    if found > lower && found <= upper {
        return 1
    }
    return 0
}

fn main() {
    let input = util::load_input(2).unwrap();

    let clean_input = input.split("\n").filter(|x| !x.is_empty());

    let mut output: Vec<u32> = Vec::new();

    for line in clean_input {
        let (raw_policy, password) = line.split_once(": ").unwrap();
        let (range, target_char_str) = raw_policy.split_once(" ").unwrap();
        let target_char = target_char_str.to_string().parse::<char>().unwrap();

        let (lower_str, upper_str) = range.split_once("-").unwrap();
        let lower = lower_str.to_string().parse::<u32>().unwrap();
        let upper = upper_str.to_string().parse::<u32>().unwrap();
        output.push(is_valid_pass(lower, upper, target_char, password));
    }
    let result: u32 = output.iter().sum();
    print!("{:?}\n", result);

    assert_eq!(solve(2020, 2, 1, &input), Ok(result.to_string()))

}
