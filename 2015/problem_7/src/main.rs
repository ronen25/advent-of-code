mod errors;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::errors::InterpreterError;

#[derive(Debug)]
enum Command<'a> {
    ValuePut(i32, &'a str),
    WirePut(&'a str, &'a str),
    And(&'a str, &'a str, &'a str),
    Or(&'a str, &'a str, &'a str),
    RShift(&'a str, &'a str, &'a str),
    LShift(&'a str, &'a str, &'a str),
}

fn parse_command<'a>(raw_line: &'a str, target: &'a str) -> Option<Command> {
    let parts: Vec<&'a str> = raw_line.split(" ").collect();

    match parts.len() {
        1 => {
            // Value put
            if let Ok(value) = parts[0].parse() {
                Some(Command::ValuePut(value, target))
            } else {
                Some(Command::WirePut(parts[0], target))
            }
        }
        2 => {
            // NOT on value
            if let Ok(value) = parts[0].parse::<i32>() {
                Some(Command::ValuePut(!value, target))
            } else {
                Some(Command::WirePut(parts[0], target))
            }
        }
        3 => {
            let command = parts[1];
            match command {
                "AND" => Some(Command::And(parts[0], parts[2], target)),
                "OR" => Some(Command::Or(parts[0], parts[2], target)),
                "RSHIFT" => Some(Command::RShift(parts[0], parts[2], target)),
                "LSHIFT" => Some(Command::LShift(parts[0], parts[2], target)),
                _ => None,
            }
        }
        _ => None,
    }
}

fn execute_command<'a>(cmd: &'a Command, state: &mut HashMap<&'a str, i32>) {
    match &cmd {
        Command::ValuePut(value, target) => {
            state.entry(*target).and_modify(|_| *value);
        }
        _ => {
            println!("Unknown command: {:?}", cmd)
        }
    }
}

fn main() -> Result<(), InterpreterError> {
    let mut file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut state: HashMap<&str, i32> = HashMap::new();

    for (line_no, line) in reader.lines().enumerate() {
        let line_raw = line.unwrap();
        let parts: Vec<&str> = line_raw.split("->").map(|item| item.trim()).collect();

        if let [command, target] = parts[..] {
            if let Some(cmd) = parse_command(command, target) {
                execute_command(&cmd, &mut state);
            } else {
                return Err(InterpreterError::InvalidCommand(
                    line_no,
                    line_raw.to_string(),
                ));
            }
        } else {
            return Err(InterpreterError::InvalidLine(line_no, line_raw.to_string()));
        }
    }

    Ok(())
}
