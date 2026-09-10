//! Ported a rock-paper-scissors game from Python

use std::io::{self, Write};
use rand::Rng;

/// One of the three possible moves in a round.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

/// The result of a round 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Lose,
    Tie,
}

/// Decides the outcome of a round 
pub fn judge(player: Move, computer: Move) -> Outcome {
    use Move::*;
    if player == computer {
        return Outcome::Tie;
    }
    match (player, computer) {
        (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Outcome::Win,
        _ => Outcome::Lose,
    }
}

/// Parses a move
pub fn parse_move(input: &str) -> Option<Move> {
    match input.trim().to_lowercase().as_str() {
        "rock" | "r" => Some(Move::Rock),
        "paper" | "p" => Some(Move::Paper),
        "scissors" | "s" => Some(Move::Scissors),
        _ => None,
    }
}

/// Picks a random `Move` using the RNG
pub fn random_move(rng: &mut impl Rng) -> Move {
    match rng.gen_range(0..3) {
        0 => Move::Rock,
        1 => Move::Paper,
        _ => Move::Scissors,
    }
}

/// Funky way to implement print for the Move struct. 
impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Move::Rock => "Rock",
            Move::Paper => "Paper",
            Move::Scissors => "Scissors",
        };
        write!(f, "{s}")
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let (mut wins, mut losses, mut ties) = (0u32, 0u32, 0u32);

    println!("Rock, Paper, Scissors! Enter rock/paper/scissors (or quit to exit).");

    loop {
        print!("Your move: ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input, exiting.");
            break;
        }

        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("quit") || trimmed.eq_ignore_ascii_case("q") {
            break;
        }

        let player_move = match parse_move(trimmed) {
            Some(m) => m,
            None => {
                println!("Unrecognized move: {trimmed:?}. Try rock, paper, or scissors.");
                continue;
            }
        };

        let computer_move = random_move(&mut rng);
        let outcome = judge(player_move, computer_move);

        match outcome {
            Outcome::Win => wins += 1,
            Outcome::Lose => losses += 1,
            Outcome::Tie => ties += 1,
        }

        let outcome_text = match outcome {
            Outcome::Win => "You win!",
            Outcome::Lose => "You lose!",
            Outcome::Tie => "It's a tie!",
        };
        println!("You chose {player_move}, computer chose {computer_move}. {outcome_text}");
        println!("Score - Wins: {wins}, Losses: {losses}, Ties: {ties}");
    }

    println!("Final score.  Wins: {wins}, Losses: {losses}, Ties: {ties}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    //Does the judge handle all cases
    fn judge_covers_win_lose_and_tie_cases() {
        assert_eq!(judge(Move::Rock, Move::Scissors), Outcome::Win);
        assert_eq!(judge(Move::Scissors, Move::Paper), Outcome::Win);
        assert_eq!(judge(Move::Paper, Move::Rock), Outcome::Win);

        assert_eq!(judge(Move::Scissors, Move::Rock), Outcome::Lose);
        assert_eq!(judge(Move::Paper, Move::Scissors), Outcome::Lose);
        assert_eq!(judge(Move::Rock, Move::Paper), Outcome::Lose);

        assert_eq!(judge(Move::Rock, Move::Rock), Outcome::Tie);
        assert_eq!(judge(Move::Paper, Move::Paper), Outcome::Tie);
        assert_eq!(judge(Move::Scissors, Move::Scissors), Outcome::Tie);
    }

    #[test]
    // properly reject bad inputs
    fn parse_move_accepts_valid_input_and_rejects_the_rest() {
        assert_eq!(parse_move("rock"), Some(Move::Rock));
        assert_eq!(parse_move("ROCK"), Some(Move::Rock));
        assert_eq!(parse_move(" p "), Some(Move::Paper));
        assert_eq!(parse_move("S"), Some(Move::Scissors));
        assert_eq!(parse_move("bruh"), None);
        assert_eq!(parse_move(""), None);
    }

    #[test]
    // random is in range
    fn random_move_always_returns_a_valid_move() {
        let mut rng = StdRng::seed_from_u64(12345);
        for _ in 0..1000 {
            let m = random_move(&mut rng);
            assert!(matches!(m, Move::Rock | Move::Paper | Move::Scissors));
        }
    }

    #[test]
    // RNG works properly
    fn random_move_is_deterministic_given_the_same_seed() {
        let mut rng_a = StdRng::seed_from_u64(42);
        let mut rng_b = StdRng::seed_from_u64(42);
        for _ in 0..50 {
            assert_eq!(random_move(&mut rng_a), random_move(&mut rng_b));
        }
    }
}
