//
// Rock, Paper, Scissors
//  A game by David Wesst, based on implementation from Riskpeep
//  Part 1 Reference: https://www.riskpeep.com/2022/07/rock-paper-scissors.html
//  Part 2 Reference: https://www.riskpeep.com/2022/07/rock-paper-scissors-game-in-rust-ii.html

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
    println!("Best 3 out of 5, okay?");

    let mut quit = false;

    let mut player_wins = 0;
    let mut cpu_wins = 0;

    'game: loop { // game 

        loop { // round
            println!("Select (r)ock, (p)aper, or (s)cissors");
            let cpu_move: RPSChoice = rand::thread_rng().gen();

            let mut player_move = String::new();

            io::stdin()
                .read_line(&mut player_move) 
                .expect("Failed to read input");


            let player_move : Result<RPSChoice, RPSChoiceError> = player_move.trim().parse();
            let player_move = match player_move {
                Ok(player_move_val) => {
                    // OK
                    println!("");
                    println!("You guessed {player_move_val}");
                    println!("I guessed {cpu_move}");
                    player_move_val
                }
                Err(RPSChoiceError::Unknown(s)) => {
                    // Err
                    match &s[..] {
                        "q" | "quit" => {
                            println!("Quitting...");
                            quit = true;
                            break 'game;
                        },
                        _ => {
                            println!("WTF dude?! {s} is not an option. Try again.");
                            continue
                        }
                    }
                }
            };

            let result: RPSResult = player_move.compare(&cpu_move);
            println!("{}", result);

            match result {
                RPSResult::Win(_) => {
                    player_wins += 1;
                    println!("You win this round.");
                },
                RPSResult::Loss(_) => {
                    cpu_wins += 1;
                    println!("I win this round");
                },
                RPSResult::Tie() => {
                    println!("Tied round...no points.")
                }
            }
            
            if player_wins == 3 {
                println!("\nCongratulations! You won the game.");
                break;
            } else if cpu_wins == 3 {
                println!("\nOh snap! You lost sucka! Better luck next time.");
                break;
            } else {
                println!("You have {player_wins} points and I have {cpu_wins} points.\n");
                print!("Again. :) ")
            }
        }
        break;
    }

    if quit == true {
        println!("Thanks for playing! Hope to see you again soon. 🙂");
    }

}
