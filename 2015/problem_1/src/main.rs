use std::fs::File;
use std::io::{Read, Result as IOResult};

fn main() -> IOResult<()> {
    let mut file = File::open("./input.txt")?;
    let mut contents = String::new();
    let mut floor = 0; // We begin at floor 0
    let mut already_counted_basement = false;

    file.read_to_string(&mut contents)?;

    for (index, character) in contents.chars().enumerate() {
        if character == '(' {
            floor += 1;
        } else if character == ')' {
            // Check if it's the first position where Santa is in the basement
            if floor == -1 && !already_counted_basement {
                println!(
                    "Position of character where santa first visits the basement: {}",
                    index
                );
                already_counted_basement = true;
            }
            floor -= 1;
        }
    }

    println!("Santa will be taken to floor: {}", floor);

    Ok(())
}
