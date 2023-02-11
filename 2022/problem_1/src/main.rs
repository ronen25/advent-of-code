fn main() {
    let contents = include_str!("../input.txt");
    let mut elf_calories: Vec<i32> = Vec::new();

    for calories_line in contents.split("\n\n") {
        let sum = calories_line
            .lines()
            .map(|line_value| line_value.parse::<i32>().unwrap_or_default())
            .sum::<i32>();

        elf_calories.push(sum);
    }

    elf_calories.sort_by(|item1, item2| item2.cmp(item1));

    let max_calories = &elf_calories[0..3];
    let sum_max: i32 = max_calories.iter().sum();

    println!("Max calories = {max_calories:?}, sum = {sum_max}");
}
