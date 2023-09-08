use clap::Parser;

use std::io;
use std::time::{Instant, Duration};
use rand::{self, Rng};

use rps;
use rps::choice::{RPSChoice, RPSChoiceError};
use rps::compare::Compare;
use rps::result::RPSResult;

mod interactive_mode;
use interactive_mode::*;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct RPSArgs {
    #[arg(short, long, default_value_t = false, help = "Run RPS in non-interactive mode.")]
    non_interactive: bool,
    #[arg(short, long, default_value_t = false, help = "Print log messages in terminal window.")]
    verbose: bool,
    #[arg(short, long, default_value_t = 3, help = "Number of wins a player needs to win the series.")]
    win_count: usize,
}

fn read_player_input() -> RPSChoice {
    let mut player_input = String::new();
    let player_move_choice: RPSChoice;

    loop {
        io::stdin()
            .read_line(&mut player_input) 
            .expect("Failed to read input");

        let choice: Result<RPSChoice, RPSChoiceError> = player_input.trim().parse();
        match choice {
            Ok(parsed_choice) => {
                player_move_choice = parsed_choice;
                break;
            }
            Err(RPSChoiceError::Unknown(s)) => {
                println!("Unknown input: {s}");
                match &s[..] {
                    "q" | "quit" => {
                        // quit = true;
                        // TODO: Handle quit event
                    },
                    _ => {
                        print_invalid_input_message(&s);
                        continue
                    }
                } 
            }
        }
    }

    return player_move_choice;
}

fn main() {
    let game_timer: Instant = Instant::now();

    let args: RPSArgs = RPSArgs::parse();

    let mut player_wins = 0;
    let mut cpu_wins = 0;
    let mut tie_count: usize = 0;

    if !args.non_interactive {
        print_game_start_message(&args);
    }

    loop { // game 
        loop { // round
            if !args.non_interactive {
                print_round_start_message();
            }

            let player_move: RPSChoice;
            if !args.non_interactive {
                player_move = read_player_input(); 
            }
            else {
                player_move = rand::thread_rng().gen();
            }
            let cpu_move: RPSChoice = rand::thread_rng().gen();
            if !args.non_interactive {
                print_round_summary_message(&player_move, &cpu_move);
            }

            let result: RPSResult = player_move.compare(&cpu_move);
            if !args.non_interactive {
                print_round_result_message(&result);
            }
            match result {
                RPSResult::Win(_) => {
                    player_wins += 1;
                },
                RPSResult::Loss(_) => {
                    cpu_wins += 1;
                },
                RPSResult::Tie() => { 
                    tie_count += 1;
                }
            }

            if player_wins == args.win_count || cpu_wins == args.win_count {
                let game_duration: Duration = game_timer.elapsed();
                if !args.non_interactive {
                    print_game_over_message(&player_wins, &cpu_wins, &tie_count, &game_duration); 
                }
                else {
                    print_game_over_message(&player_wins, &cpu_wins, &tie_count, &game_duration);
                }
                break;
            }
            else {
                if !args.non_interactive {
                    print_game_summary_message(&player_wins, &cpu_wins);
                }
            }
        }
        break;
    }

}
