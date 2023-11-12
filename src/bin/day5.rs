use advent_of_code::solve;
use aoc_2020_rust::util;

enum Direction {
    Lower,
    Higher,
}
fn row_to_direction(c: char) -> Direction {
    match c {
        'F' => Direction::Lower,
        'B' => Direction::Higher,
        _ => panic!("Found unexpected char {c}"),
    }
}
fn col_to_direction(c: char) -> Direction {
    match c {
        'L' => Direction::Lower,
        'R' => Direction::Higher,
        _ => panic!("Found unexpected char {c}"),
    }
}
fn bin_search(start: u32, stop: u32, bin_partitioning: &Vec<Direction>) -> u32 {
    let mut low = start;
    let mut high = stop;
    let mut mid = (high - low) / 2;
    for direction in bin_partitioning {
        match direction {
            Direction::Lower => {
                high = mid;
                mid = low + (high - low) / 2;
            }
            Direction::Higher => {
                low = mid + 1;
                mid = low + (high - low) / 2;
            }
        }
    }
    assert_eq!(low, high);
    low
}

fn main() {
    let input = util::load_input(5).unwrap();
    let char_instructions: Vec<(&str, &str)> =
        input.split_whitespace().map(|x| x.split_at(7)).collect();

    let bin_partitioning: Vec<(Vec<Direction>, Vec<Direction>)> = char_instructions
        .iter()
        .map(|(row_str, col_str)| {
            (
                row_str
                    .chars()
                    .map(|row_char| row_to_direction(row_char))
                    .collect(),
                col_str
                    .chars()
                    .map(|col_char| col_to_direction(col_char))
                    .collect(),
            )
        })
        .collect();

    let mut seat_ids: Vec<u32> = Vec::<u32>::new();
    for (row_bin_part, col_bin_part) in bin_partitioning {
        let row = bin_search(0, 127, &row_bin_part);
        let col = bin_search(0, 7, &col_bin_part);
        let seat_id = row * 8 + col;
        seat_ids.push(seat_id);
    }
    seat_ids.sort();
    let highest = seat_ids[seat_ids.len() - 1];
    print!("Part 1 solution: {highest}\n");
    assert_eq!(solve(2020, 5, 1, &input), Ok(highest.to_string()));

    // Part 2
    let mut i = 0;
    let mut found = false;

    while i < seat_ids.len() - 1 && !found {
        if seat_ids[i + 1] - seat_ids[i] != 1 {
            found = true;
        }
        i += 1;
    }
    let our_seat_id = seat_ids[i] - 1;
    print!("Part two solution {}\n", our_seat_id);

    assert_eq!(solve(2020, 5, 2, &input), Ok(our_seat_id.to_string()));
}
