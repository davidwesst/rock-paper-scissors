//
// Rock, Paper, Scissors
//  A game by David Wesst, based on implementation from Riskpeep
//  Part 1 Reference: https://www.riskpeep.com/2022/07/rock-paper-scissors.html
//  Part 2 Reference: https://www.riskpeep.com/2022/07/rock-paper-scissors-game-in-rust-ii.html
use std::io;
use rand::{self, Rng};

use rps;
use rps::choice::{RPSChoice, RPSChoiceError};
use rps::compare::Compare;
use rps::result::RPSResult;

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
