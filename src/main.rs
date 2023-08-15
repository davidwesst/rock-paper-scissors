//
// Rock, Paper, Scissors
//  A game by David Wesst, based on implementation from Riskpeep
//  Reference: https://www.riskpeep.com/2022/07/rock-paper-scissors.html

use std::{fmt::{self},io};
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

#[derive(Debug)]
enum RPSChoiceError {
    Unknown(String),
}

impl core::str::FromStr for RPSChoice {
    type Err = RPSChoiceError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" | "rock"    => Ok(RPSChoice::Rock),
            "p" | "paper"   => Ok(RPSChoice::Paper),
            "s" | "scissors"=> Ok(RPSChoice::Scissors),
            _ => Err(RPSChoiceError::Unknown(s.to_string())),
        }
    }
}

enum RPSCompare {
    RockCrushesScissors,
    PaperCoversRock,
    ScissorsCutsPaper
}

enum RPSResult {
    Win(RPSCompare),
    Loss(RPSCompare),
    Tie(),
}

pub trait Compare<T, U> {
    fn compare(&self, b: &T) -> U;
}

impl Compare<RPSChoice, RPSResult> for RPSChoice {
    fn compare(&self, b: &RPSChoice) -> RPSResult {
        match self {
            RPSChoice::Rock => {
                match b {
                    RPSChoice::Rock => RPSResult::Tie(),
                    RPSChoice::Paper => RPSResult::Loss(RPSCompare::PaperCoversRock),
                    RPSChoice::Scissors => RPSResult::Win(RPSCompare::RockCrushesScissors)
                }
            },
            RPSChoice::Paper => {
                match b {
                    RPSChoice::Rock => RPSResult::Win(RPSCompare::PaperCoversRock),
                    RPSChoice::Paper => RPSResult::Tie(),
                    RPSChoice::Scissors => RPSResult::Loss(RPSCompare::ScissorsCutsPaper)
                }
            },
            RPSChoice::Scissors => {
                match b {
                    RPSChoice::Rock => RPSResult::Loss(RPSCompare::RockCrushesScissors),
                    RPSChoice::Paper => RPSResult::Win(RPSCompare::ScissorsCutsPaper),
                    RPSChoice::Scissors => RPSResult::Tie(),
                }
            }
        }
    }
}

impl fmt::Display for RPSResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RPSResult::Win(result) => {
                match result {
                    RPSCompare::PaperCoversRock => write!(f, "You win...Paper covers rock!"),
                    RPSCompare::ScissorsCutsPaper => write!(f, "You win...Scissors cuts paper!"),
                    RPSCompare::RockCrushesScissors => write!(f, "You win...Rock crushes scissors!")
                }
            },
            RPSResult::Loss(result) => {
                match result {
                    RPSCompare::PaperCoversRock => write!(f, "You lose...Paper covers rock!"),
                    RPSCompare::ScissorsCutsPaper => write!(f, "You lose...Scissors cuts paper!"),
                    RPSCompare::RockCrushesScissors => write!(f, "You lose...Rock crushes scissors!")
                }
            },
            RPSResult::Tie() => write!(f, ""),
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

    let player_move : RPSChoice = 
        player_move.trim().parse().expect("This is not a valid option.");

    println!("You guessed {player_move}");

    let cpu_move: RPSChoice = rand::thread_rng().gen();
    println!("I guessed {cpu_move}");

    let result: RPSResult = player_move.compare(&cpu_move);
    println!("{}", result);
}
