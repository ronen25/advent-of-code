#[derive(PartialEq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn value(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err(format!("Unknown move value '{value}'")),
        }
    }
}
