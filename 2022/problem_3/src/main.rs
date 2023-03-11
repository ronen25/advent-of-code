mod part1;
mod part2;
mod priority;

// use part1::*;
use part2::*;

fn main() {
    let contents = include_str!("../input.txt");

    // part1(contents);
    part2(contents);
}
