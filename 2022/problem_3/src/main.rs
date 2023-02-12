mod part1;
mod priority;

use part1::*;

fn main() {
    let contents = include_str!("../input.txt");

    part1(contents);
}
