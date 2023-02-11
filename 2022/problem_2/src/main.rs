mod moves;

use moves::Move;

const PLAYER_LOSS: i32 = 0;
const PLAYER_DRAW: i32 = 3;
const PLAYER_VICTORY: i32 = 6;

fn main() {
    let strategies = include_str!("../input.txt");
    let mut score = 0;

    for turn in strategies.lines() {
        let mut outcome_points = 0;
        let moves = &(turn.split(" ").collect::<Vec<&str>>())[0..=1];

        // Yes yes I know unwrap etc.
        let their_move = Move::try_from(moves[0]).unwrap();
        let my_move = Move::try_from(moves[1]).unwrap();

        // Play the game
        if their_move == my_move {
            // Draw
            outcome_points = PLAYER_DRAW;
        } else if ((their_move == Move::Rock && my_move == Move::Paper)
            || (their_move == Move::Scissors && my_move == Move::Rock)
            || (their_move == Move::Paper && my_move == Move::Scissors))
        {
            // I won
            outcome_points = PLAYER_VICTORY;
        } else {
            // I lost
            outcome_points = PLAYER_LOSS;
        }

        score += my_move.value() + outcome_points;
    }

    println!("Final score: {score}");
}
