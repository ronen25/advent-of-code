fn main() {
    let contents = include_str!("../input.txt");
    let mut elf_calories: Vec<i32> = Vec::new();

    for calories_line in contents.split("\n\n") {
        let sum = calories_line
            .lines()
            .map(|line_value| line_value.parse::<i32>().unwrap_or_default())
            .reduce(|acc, value| acc + value)
            .unwrap_or(0 as i32);

        elf_calories.push(sum);
    }

    let max_calories = elf_calories.iter().max().take().unwrap_or(&0);

    println!("Max calories = {max_calories}");
}
