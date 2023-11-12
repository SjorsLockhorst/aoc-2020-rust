use advent_of_code::solve;
use aoc_2020_rust::util;
use regex::Regex;

fn part_one(input: &String) -> u32 {
    let expected_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let clean_input: Vec<Vec<&str>> = split_input
        .iter()
        .map(|raw_password| {
            raw_password
                .split_whitespace()
                .map(|key_value| key_value.split(":").next().unwrap())
                .collect()
        })
        .collect();

    let mut n_valid_passports = 0;
    for passport in clean_input.iter() {
        if expected_keys.iter().all(|expected_key| {
            passport
                .iter()
                .any(|passport_key| expected_key == passport_key)
        }) {
            n_valid_passports += 1;
        }
    }
    n_valid_passports
}

fn part_two(input: &String) -> u32 {
    let expected_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let clean_input: Vec<Vec<(&str, &str)>> = split_input
        .iter()
        .map(|raw_password| {
            raw_password
                .split_whitespace()
                .map(|key_value| key_value.split_once(":").unwrap())
                .collect()
        })
        .collect();

    let mut n_valid_passports = 0;

    for passport in clean_input.iter() {
        let all_fields_present: bool = expected_keys.iter().all(|expected_key| {
            passport
                .iter()
                .any(|(pass_key, _)| expected_key == pass_key)
        });
        let mut n_valid_fields = 0;

        let heigth_regex = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
        let hair_color_regex = Regex::new(r"^#([0-9]|[a-f]){6}$").unwrap();
        let passport_number_regex = Regex::new(r"^[0-9]{9}$").unwrap();

        let allowed_hair_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        for (pass_key, pass_value) in passport.iter() {
            match &pass_key as &str {
                "byr" => {
                    if pass_value.len() == 4 {
                        match pass_value.parse::<u32>() {
                            Ok(year) => {
                                if year >= 1920 && year <= 2002 {
                                    n_valid_fields += 1;
                                }
                            }
                            Err(_) => {}
                        };
                    }
                }
                "iyr" => {
                    if pass_value.len() == 4 {
                        match pass_value.parse::<u32>() {
                            Ok(year) => {
                                if year >= 2010 && year <= 2020 {
                                    n_valid_fields += 1;
                                }
                            }
                            Err(_) => {}
                        };
                    }
                }
                "eyr" => {
                    if pass_value.len() == 4 {
                        match pass_value.parse::<u32>() {
                            Ok(year) => {
                                if year >= 2020 && year <= 2030 {
                                    n_valid_fields += 1;
                                }
                            }
                            Err(_) => {}
                        };
                    }
                }
                "hgt" => match heigth_regex.captures(pass_value) {
                    Some(caps) => {
                        let str_heigth = caps.get(1).unwrap().as_str();
                        let str_metric = caps.get(2).unwrap().as_str();
                        let heigth = str_heigth.parse::<u32>().unwrap();

                        match str_metric.as_ref() {
                            "cm" => {
                                if heigth >= 150 && heigth <= 193 {
                                    n_valid_fields += 1;
                                }
                            }
                            "in" => {
                                if heigth >= 59 && heigth <= 76 {
                                    n_valid_fields += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                    None => {}
                },
                "hcl" => {
                    if hair_color_regex.is_match(pass_value) {
                        n_valid_fields += 1;
                    }
                }
                "ecl" => {
                    if allowed_hair_colors.iter().any(|color| color == pass_value) {
                        n_valid_fields += 1;
                    }
                }
                "pid" => {
                    if passport_number_regex.is_match(pass_value) {
                        n_valid_fields += 1;
                    }
                }
                _ => {}
            }
        }
        if all_fields_present && n_valid_fields == 7 {
            n_valid_passports += 1;
        }
    }
    n_valid_passports
}

fn main() {
    let input = util::load_input(4).unwrap();

    let n_valid_passports = part_one(&input);

    assert_eq!(solve(2020, 4, 1, &input), Ok(n_valid_passports.to_string()));
    print!("Part one solution: {n_valid_passports}\n");

    let valid_two = part_two(&input);
    assert_eq!(solve(2020, 4, 2, &input), Ok(valid_two.to_string()));
    print!("Part two solution: {valid_two}\n");
}
