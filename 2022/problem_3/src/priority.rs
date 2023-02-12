pub fn get_priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 96,
        'A'..='Z' => item as u32 - 38,
        _ => 0,
    }
}
