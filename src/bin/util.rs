use std::fs;
use std::io;


pub fn load_input(day: u32) -> io::Result<String> {
    fs::read_to_string(format!("./inputs/day{}.txt", day))
}

pub fn main() {
}
