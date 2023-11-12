use advent_of_code::solve;
use aoc_2020_rust::util;

fn traverse_forest(forest: &Vec<Vec<u8>>, dx: usize, dy: usize) -> u64 {
    let forest_width = forest[0].len();

    let mut n_trees_in_path: u64 = 0;
    let mut x = 0;
    let mut y = 0;

    while y < forest.len() {
        let col = x % (forest_width);
        let val = forest[y][col];
        n_trees_in_path += val as u64;

        x += dx;
        y += dy;
    }
    n_trees_in_path
}

fn main() {
    // Parsing of input
    let input = util::load_input(3).unwrap();
    let forest: Vec<Vec<u8>> = input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.chars()
                .map(|c| {
                    if c == '#' {
                        1
                    } else if c == '.' {
                        0
                    } else {
                        panic!("Found unexpected char")
                    }
                })
                .collect()
        })
        .collect();

    let solution_part_one = traverse_forest(&forest, 3, 1);
    assert_eq!(solve(2020, 3, 1, &input), Ok(solution_part_one.to_string()));
    print!("Part one: {}\n", solution_part_one);

    let slopes: Vec<(usize, usize)> = vec![(1, 1), (5, 1), (7, 1), (1, 2)];
    let mut solution_part_two: u64 = slopes
        .iter()
        .map(|(dx, dy)| traverse_forest(&forest, *dx, *dy))
        .product();
    solution_part_two *= solution_part_one;

    print!("Part two {solution_part_two:?}\n");
    assert_eq!(solve(2020, 3, 2, &input), Ok(solution_part_two.to_string()));
}
