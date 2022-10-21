use std::fs::File;
use std::io::{prelude::*, BufReader, Error as IOError};
use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
enum ProgramError {
    #[error("I/O Error")]
    IO(#[from] IOError),

    #[error("Parse Error")]
    Parse(#[from] ParseIntError),

    #[error("")]
    ValueError,
}

const fn surface_area(width: i32, height: i32, length: i32) -> i32 {
    return 2 * length * width + 2 * width * height + 2 * height * length;
}

fn main() -> Result<(), ProgramError> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut total_area = 0;
    let mut total_ribbon = 0;

    for line in reader.lines() {
        let line_value = line.unwrap();
        let mut parts: Vec<i32> = line_value
            .split('x')
            .map(|elem| elem.parse::<i32>().unwrap())
            .collect();

        if parts.len() != 3 {
            eprintln!("Expected 3 parts (LxWxH), got {}", parts.len());
            return Err(ProgramError::ValueError);
        }

        // Calculate area
        let [length, width, height] = [parts[0], parts[1], parts[2]];
        parts.sort();
        let [min_1, min_2] = [parts[0], parts[1]];

        let area = surface_area(width, height, length);
        let small_extra_area = min_1 * min_2;

        total_area += area + small_extra_area;

        // Calculate ribbon
        let ribbon_base = 2 * min_1 + 2 * min_2;
        let ribbon_tie = length * width * height;

        total_ribbon += ribbon_base + ribbon_tie;
    }

    println!("Total area: {}", total_area);
    println!("Total ribbon: {}", total_ribbon);

    Ok(())
}
