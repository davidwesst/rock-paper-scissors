use std::fmt::{self};

use super::compare::RPSCompare;

#[derive(Debug, PartialEq)]
pub enum RPSResult {
    Win(RPSCompare),
    Loss(RPSCompare),
    Tie(),
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
            RPSResult::Tie() => write!(f, "Tie."),
        }
    }
}