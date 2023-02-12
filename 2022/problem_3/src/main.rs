use std::collections::HashSet;

fn get_priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 96,
        'A'..='Z' => item as u32 - 38,
        _ => 0,
    }
}

fn main() {
    let contents = include_str!("../input.txt");
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
