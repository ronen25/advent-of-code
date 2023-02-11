mod moves;
mod roundresult;

use moves::Move;
use roundresult::RoundResult;

fn main() {
    let strategies = include_str!("../input.txt");
    let mut score = 0;

    for turn in strategies.lines() {
        let plays = &(turn.split(" ").collect::<Vec<&str>>())[0..=1];

        // Yes yes I know unwrap etc.
        let their_move = Move::try_from(plays[0]).unwrap();
        let expected_round_result = RoundResult::try_from(plays[1]).unwrap();
        let my_move: Move;

        // Play the game
        if expected_round_result == RoundResult::Draw {
            // Choose the same move
            my_move = their_move;
        } else if expected_round_result == RoundResult::Victory {
            my_move = match their_move {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            };
        } else {
            my_move = match their_move {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            };
        }

        score += my_move.value() + expected_round_result.value();
    }

    println!("Final score: {score}");
}
