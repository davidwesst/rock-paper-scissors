
use std::io;
use rand::{self, Rng};

use rps;
use rps::choice::{RPSChoice, RPSChoiceError};
use rps::compare::Compare;
use rps::result::RPSResult;

use super::RPSArgs;

pub fn run_interactive_mode(args: &RPSArgs) {
    println!("Let's play Rock, Paper, Scissors!");
    println!("First one to win {} takes it all, okay?", args.win_count);

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
            
            if player_wins == args.win_count {
                println!("\nCongratulations! You won the game.");
                break;
            } else if cpu_wins == args.win_count {
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