use advent_of_code::solve;
use aoc_2020_rust::util;

use std::{collections::HashSet};

fn main() {
    let input = util::load_input(6).unwrap();
    let groups: Vec<Vec<&str>> = input
        .split("\n\n")
        .map(|x| x.split_whitespace().collect())
        .collect();

    let mut total_count = 0;
    for group in groups.iter() {
        let mut group_questions: HashSet<char> = HashSet::new();

        for persons_questions in group {
            let mut person_set: HashSet<char> = HashSet::new();
            for char in persons_questions.chars() {
                person_set.insert(char);
            }
            group_questions = group_questions.union(&person_set).cloned().collect();
        }
        total_count += group_questions.len();
    }

    assert_eq!(solve(2020, 6, 1, &input), Ok(total_count.to_string()));
    print!("Part one: {total_count}\n");

    let mut total_count = 0;
    for group in groups {
        let allowed_chars = "abcdefghijklmnopqrstuvwxyz";
        let mut group_questions: HashSet<char> = HashSet::new();
        for char in allowed_chars.chars() {
            group_questions.insert(char);
        }

        for persons_questions in group {
            let mut person_set: HashSet<char> = HashSet::new();
            for char in persons_questions.chars() {
                person_set.insert(char);
            }
            group_questions = group_questions.intersection(&person_set).cloned().collect();
        }
        total_count += group_questions.len()
    }
    assert_eq!(solve(2020, 6, 2, &input), Ok(total_count.to_string()));
    print!("Part one: {total_count}\n");
}
