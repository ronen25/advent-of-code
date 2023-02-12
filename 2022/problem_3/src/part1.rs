use crate::priority::*;

use std::collections::HashSet;

pub fn part1(contents: &str) {
    let mut sum = 0;

    for line in contents.lines() {
        let (first, second) = line.split_at(line.len() / 2);

        let contents_1: HashSet<char> = first.chars().collect();
        let contents_2: HashSet<char> = second.chars().collect();

        let shared_item: char = *contents_1.intersection(&contents_2).collect::<Vec<&char>>()[0];
        let value = get_priority(shared_item);

        println!("Got: ({shared_item}) -> {value}");

        sum += value;
    }

    println!("Sum = {}", sum);
}
