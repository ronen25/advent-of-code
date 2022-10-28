use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Result as IOResult};

fn main() -> IOResult<()> {
    let mut file = File::open("./input.txt")?;
    let mut input = String::new();

    file.read_to_string(&mut input)?;

    let mut santa_houses: HashMap<(i32, i32), i32> = HashMap::new();
    let mut robot_houses: HashMap<(i32, i32), i32> = HashMap::new();
    let mut coords = [(0, 0), (0, 0)]; // Coordinates for the house currently visited by santa (0) and the robot (1)

    santa_houses.insert((0, 0), 1); // First house always gets delivered to

    for (index, direction) in input.char_indices() {
        let mover = if index % 2 == 0 { 0 } else { 1 };

        match direction {
            '>' => {
                coords[mover] = (coords[mover].0, coords[mover].1 + 1);
            }
            '<' => {
                coords[mover] = (coords[mover].0, coords[mover].1 - 1);
            }
            '^' => {
                coords[mover] = (coords[mover].0 - 1, coords[mover].1);
            }
            'v' => {
                coords[mover] = (coords[mover].0 + 1, coords[mover].1);
            }
            _ => {
                println!("Unknown pattern: {}", direction);
            }
        }

        // Add the houses
        if mover == 0 {
            santa_houses
                .entry(coords[0])
                .and_modify(|count| {
                    *count += 1;
                })
                .or_insert(1);
        } else {
            robot_houses
                .entry(coords[1])
                .and_modify(|count| {
                    *count += 1;
                })
                .or_insert(1);
        }
    }

    // Combine both house coords
    santa_houses.extend(robot_houses);

    println!("{}", santa_houses.len());

    Ok(())
}
