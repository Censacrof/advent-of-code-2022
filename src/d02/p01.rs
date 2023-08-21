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
    let move_lookup: HashMap<char, Move> = HashMap::from([
        ('A', Rock),
        ('B', Paper),
        ('C', Scissor),
        ('X', Rock),
        ('Y', Paper),
        ('Z', Scissor),
    ]);

    let outcome_lookup: HashMap<(Move, Move), Outcome> = HashMap::from([
        ((Rock, Rock), Draw),
        ((Rock, Paper), Win),
        ((Rock, Scissor), Lose),
        ((Paper, Rock), Lose),
        ((Paper, Paper), Draw),
        ((Paper, Scissor), Win),
        ((Scissor, Rock), Win),
        ((Scissor, Paper), Lose),
        ((Scissor, Scissor), Draw),
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
        let our_char = line.chars().nth(2).unwrap();

        let opponent_move = move_lookup.get(&opponent_char).unwrap();
        let our_move = move_lookup.get(&our_char).unwrap();

        let outcome = outcome_lookup.get(&(*opponent_move, *our_move)).unwrap();

        let round_score = score_lookup.get(&OutcomeScore(outcome.clone())).unwrap()
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
        assert_eq!(res, 15)
    }
}
