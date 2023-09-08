
use std::time::Duration;

use rps;
use rps::choice::RPSChoice;
use rps::result::RPSResult;

use super::RPSArgs;

pub fn print_game_start_message(args: &RPSArgs) {
    println!("Let's play Rock, Paper, Scissors!");
    println!("First one to win {} takes it all, okay?", args.win_count);
}

pub fn print_round_start_message() {
    println!("Select (r)ock, (p)aper, or (s)cissors");
}

pub fn print_round_summary_message(p1_move: &RPSChoice, p2_move: &RPSChoice) {
    println!("\nYou guessed {p1_move}");
    println!("I guessed {p2_move}");
}

pub fn print_round_result_message(result: &RPSResult) {
    println!("{}", result);
}

pub fn print_game_over_message(p1_score:&usize, p2_score:&usize, tie_count:&usize, game_duration: &Duration) {
    if p1_score > p2_score {
        println!("You win!");
    }
    else if p2_score > p1_score {
        println!("Oh snap! I win!");
    }
    else {
        println!("It's a draw. You should play again so I can beat you. 😊");
    }

    println!("\n---- FINAL SCORE ----\nP1 (You): {}\nP2 (CPU): {}\nTies: {}", p1_score, p2_score, tie_count);
    println!("\nGame lasted {} seconds and had {} rounds.", game_duration.as_secs(), p1_score + p2_score + tie_count);
}

pub fn print_game_summary_message(p1_score:&usize, p2_score:&usize) {
    println!("You have {p1_score} points and I have {p2_score} points.\n");
    print!("Again. :) ")
}

#[allow(dead_code)]
pub fn print_game_exit_message() {
    println!("Quitting the game...");
    println!("Thanks for playing! Hope to see you again soon. 🙂");
}

pub fn print_invalid_input_message(s: &String) {
    println!("WTF dude?! {s} is not an option. Try again.");
}
