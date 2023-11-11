use std::fs;
use std::io;

use advent_of_code::solve;

pub fn load_input(day: u32, part: u32) -> io::Result<String> {
    fs::read_to_string(format!("./inputs/day{}_{}.txt", day, part))
}

fn part_one(nums: &Vec<u32>) -> u32 {
    let mut solution: u32 = 0;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            let a = nums[i];
            let b = nums[j];

            let sum = &a + &b;
            if sum == 2020 as u32 {
                solution = &a * &b;
            }
        }
    }
    solution
}

fn part_two(nums: &Vec<u32>) -> u32 {
    let mut solution: u32 = 0;
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            for k in 0..nums.len() {
                let a = nums[i];
                let b = nums[j];
                let c = nums[k];

                let sum = &a + &b + &c;
                if sum == 2020 as u32 {
                    solution = &a * &b * &c;
                }
            }
        }
    }
    solution
}

fn main() {
    let input = load_input(1, 1).unwrap();

    let nums: Vec<u32> = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .into_iter()
        .map(|x| x.to_string().parse::<u32>().unwrap())
        .collect();

    let ans_part_one = part_one(&nums);
    assert_eq!(solve(2020, 1, 1, &input), Ok(ans_part_one.to_string()));
    print!("Passed part one!\n Answer {}.\n", ans_part_one);

    let ans_part_two = part_two(&nums);
    assert_eq!(solve(2020, 1, 2, &input), Ok(ans_part_two.to_string()));
    print!("Passed part two!\n Answer {}.\n", ans_part_two);
}
