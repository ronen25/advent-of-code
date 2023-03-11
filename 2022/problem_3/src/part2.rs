use std::collections::HashSet;

use crate::priority::*;

pub fn part2(contents: &str) {
    let all_lines: Vec<&str> = contents.lines().collect();
    for line_group in all_lines.windows(3) {
        let mut dictionary: HashSet<char> = HashSet::new();

        let line_chars: Vec<HashSet<char>> = line_group
            .iter()
            .map(|line| line.chars().collect::<HashSet<char>>())
            .collect();

        let line_chars_iter = line_chars.iter();

        println!("{line_chars_iter:?}");
    }
}
