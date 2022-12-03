#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Result {
    Win,
    Lose,
    Draw
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
    Empty
}

// TODO: could probably use better names
#[derive(Debug)]
pub struct Choice {
    pub choice: Hand
}

impl Hand {
    pub fn play_against(&self, other: Hand) -> Result {
        match (self, other) {
            (Hand::Rock, Hand::Paper) => Result::Win,
            (Hand::Rock, Hand::Rock) => Result::Draw,
            (Hand::Rock, Hand::Scissors) => Result::Lose,
            (Hand::Scissors, Hand::Scissors) => Result::Draw,
            (Hand::Scissors, Hand::Rock) => Result::Win,
            (Hand::Scissors, Hand::Paper) => Result::Lose,
            (Hand::Paper, Hand::Paper) => Result::Draw,
            (Hand::Paper, Hand::Rock) => Result::Lose,
            (Hand::Paper, Hand::Scissors) => Result::Win,
            _ => Result::Draw
        }
    }
}