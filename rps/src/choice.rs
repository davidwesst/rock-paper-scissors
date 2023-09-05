
use std::fmt::{self};
use rand::{self, Rng, distributions::{Distribution, Standard}};
use super::result::RPSResult;
use super::compare::{Compare,RPSCompare};

pub enum RPSChoice {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug)]
pub enum RPSChoiceError {
    Unknown(String),
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

