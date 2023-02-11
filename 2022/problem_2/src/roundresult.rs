#[derive(PartialEq)]
pub enum RoundResult {
    Victory,
    Draw,
    Loss,
}

impl RoundResult {
    pub fn value(&self) -> i32 {
        match self {
            RoundResult::Victory => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        }
    }
}

impl TryFrom<&str> for RoundResult {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(RoundResult::Loss),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Victory),
            _ => Err(format!("Unknown round result value '{value}'")),
        }
    }
}
