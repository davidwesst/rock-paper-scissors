//
// Rock, Paper, Scissors
//  A game by David Wesst, based on implementation from Riskpeep
//  Reference: https://www.riskpeep.com/2022/07/rock-paper-scissors.html

use std::{fmt::{self, write},io};
use rand::{self, Rng, distributions::{Distribution, Standard}};

enum RPSChoice {
    Rock,
    Paper,
    Scissors
}

impl Distribution<RPSChoice> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RPSChoice {
        let index: u8 = rng.gen_range(0..3);
        match index {
            0 => RPSChoice::Rock,
            1 => RPSChoice::Paper,
            2 => RPSChoice::Scissors,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for RPSChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RPSChoice::Rock => write!(f, "Rock"),
            RPSChoice::Paper => write!(f, "Paper"),
            RPSChoice::Scissors => write!(f, "Scissors")
        }
    }
}

fn main() {
    println!("Let's play Rock, Paper, Scissors!");
    println!("Select (r)ock, (p)aper, or (s)cissors");

    let mut player_move = String::new();

    io::stdin()
        .read_line(&mut player_move)
        .expect("Failed to read input");

    println!("You guessed {player_move}");

    let cpu_move: RPSChoice = rand::thread_rng().gen();
    println!("I guessed {cpu_move}");
}
