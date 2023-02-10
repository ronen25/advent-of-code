use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InterpreterError {
    #[error("IO Error")]
    IOError(#[from] io::Error),

    #[error("Error on line {0}: Invalid command `{1}`")]
    InvalidCommand(usize, String),

    #[error("Error on line {0}: Invalid line format `{1}`")]
    InvalidLine(usize, String),
}
