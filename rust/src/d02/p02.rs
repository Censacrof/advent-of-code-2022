use std::{
    collections::HashMap,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let mut buffer: String = String::new();
    let _bytes_read = io::stdin().read_to_string(&mut buffer);

    let result = calculate_score(&buffer);

    println!("{:?}", result);

    Ok(())
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

use Move::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

use Outcome::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum ScoreLookupKey {
    MoveScore(Move),
    OutcomeScore(Outcome),
}

use ScoreLookupKey::*;

fn calculate_score(input: &str) -> i32 {
    let move_lookup: HashMap<char, Move> =
        HashMap::from([('A', Rock), ('B', Paper), ('C', Scissor)]);

    let desired_outcome_lookup: HashMap<char, Outcome> =
        HashMap::from([('X', Lose), ('Y', Draw), ('Z', Win)]);

    let outcome_lookup: HashMap<Move, (Move, Move, Move)> = HashMap::from([
        (Rock, (Paper, Rock, Scissor)),
        (Paper, (Scissor, Paper, Rock)),
        (Scissor, (Rock, Scissor, Paper)),
    ]);

    let score_lookup: HashMap<ScoreLookupKey, i32> = HashMap::from([
        (MoveScore(Rock), 1),
        (MoveScore(Paper), 2),
        (MoveScore(Scissor), 3),
        (OutcomeScore(Lose), 0),
        (OutcomeScore(Draw), 3),
        (OutcomeScore(Win), 6),
    ]);

    let lines = input.lines();

    let mut tot = 0;
    for line in lines {
        let opponent_char = line.chars().nth(0).unwrap();
        let desired_outcome_char = line.chars().nth(2).unwrap();

        let opponent_move = move_lookup.get(&opponent_char).unwrap();
        let desired_outcome = desired_outcome_lookup.get(&desired_outcome_char).unwrap();

        let our_move: Move = match desired_outcome {
            Lose => outcome_lookup.get(opponent_move).unwrap().2,
            Draw => outcome_lookup.get(opponent_move).unwrap().1,
            Win => outcome_lookup.get(opponent_move).unwrap().0,
        };

        let round_score = score_lookup
            .get(&OutcomeScore(desired_outcome.clone()))
            .unwrap()
            + score_lookup.get(&MoveScore(our_move.clone())).unwrap();

        tot += round_score;
    }

    return tot;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate_score() {
        let res = calculate_score("A Y\nB X\nC Z\n");
        assert_eq!(res, 12)
    }
}
