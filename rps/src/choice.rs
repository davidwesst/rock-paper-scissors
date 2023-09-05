
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_loses_to_paper() {
        // given
        let rock_choice: RPSChoice = RPSChoice::Rock;

        // when
        let actual: RPSResult = rock_choice.compare(&RPSChoice::Paper);

        // then
        let expected: RPSResult = RPSResult::Loss(RPSCompare::PaperCoversRock);
        assert_eq!(expected, actual);
    }

    #[test]
    fn rock_ties_on_rock() {
        let rock_choice: RPSChoice = RPSChoice::Rock;

        let actual: RPSResult = rock_choice.compare(&RPSChoice::Rock);

        let expected: RPSResult = RPSResult::Tie();
        assert_eq!(expected, actual);
    }

    #[test]
    fn rock_wins_on_scissors() {
        let rock_choice: RPSChoice = RPSChoice::Rock;

        let actual: RPSResult = rock_choice.compare(&RPSChoice::Scissors);

        let expected: RPSResult = RPSResult::Win(RPSCompare::RockCrushesScissors);
        assert_eq!(expected, actual);
    }

    #[test]
    fn paper_loses_to_scissors() {
        let paper_choice: RPSChoice = RPSChoice::Paper;

        let actual: RPSResult = paper_choice.compare(&RPSChoice::Scissors);

        let expected: RPSResult = RPSResult::Loss(RPSCompare::ScissorsCutsPaper);
        assert_eq!(expected, actual);
    }

    #[test]
    fn paper_ties_on_paper() {
        let paper_choice: RPSChoice = RPSChoice::Paper;

        let actual: RPSResult = paper_choice.compare(&RPSChoice::Paper);

        let expected: RPSResult = RPSResult::Tie();
        assert_eq!(expected, actual);
    }

    #[test]
    fn paper_wins_on_rock() {
        let paper_choice: RPSChoice = RPSChoice::Paper;

        let actual: RPSResult = paper_choice.compare(&RPSChoice::Rock);

        let expected: RPSResult = RPSResult::Win(RPSCompare::PaperCoversRock);
        assert_eq!(expected, actual);
    }

    #[test]
    fn scissors_loses_to_rock() {
        let scissors_choice: RPSChoice = RPSChoice::Scissors;

        let actual: RPSResult = scissors_choice.compare(&RPSChoice::Rock);

        let expected: RPSResult = RPSResult::Loss(RPSCompare::RockCrushesScissors);
        assert_eq!(expected, actual);
    }

    #[test]
    fn scissors_ties_on_scissors() {
        let scissors_choice: RPSChoice = RPSChoice::Scissors;

        let actual: RPSResult = scissors_choice.compare(&RPSChoice::Scissors);

        let expected: RPSResult = RPSResult::Tie();
        assert_eq!(expected, actual);
    }

    #[test]
    fn scissors_wins_on_paper() {
        let scissors_choice: RPSChoice = RPSChoice::Scissors;

        let actual: RPSResult = scissors_choice.compare(&RPSChoice::Paper);

        let expected: RPSResult = RPSResult::Win(RPSCompare::ScissorsCutsPaper);
        assert_eq!(expected, actual);

    }
}