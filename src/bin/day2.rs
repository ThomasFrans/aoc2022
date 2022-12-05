use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Eq, PartialEq)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<u8> for RPSChoice {
    type Error = ();

    fn try_from(char: u8) -> Result<RPSChoice, Self::Error> {
        match char {
            b'A' | b'X' => Ok(Self::Rock),
            b'B' | b'Y' => Ok(Self::Paper),
            b'C' | b'Z' => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl RPSChoice {
    pub fn would_win_from(choice: RPSChoice) -> RPSChoice {
        match choice {
            RPSChoice::Rock => Self::Paper,
            RPSChoice::Paper => RPSChoice::Scissors,
            RPSChoice::Scissors => Self::Rock,
        }
    }

    pub fn would_lose_from(choice: RPSChoice) -> RPSChoice {
        match choice {
            RPSChoice::Rock => Self::Scissors,
            RPSChoice::Paper => Self::Rock,
            RPSChoice::Scissors => Self::Paper,
        }
    }
}

struct Game {
    player1: RPSChoice,
    player2: RPSChoice,
}

impl Game {
    fn _player1_score(&self) -> u32 {
        let mut score = 0;
        score += match self.player1 {
            RPSChoice::Rock => 1,
            RPSChoice::Paper => 2,
            RPSChoice::Scissors => 3,
        };
        if self.player1 == self.player2 {
            score += 3;
        } else if (self.player1 == RPSChoice::Rock && self.player2 == RPSChoice::Scissors)
            || (self.player1 == RPSChoice::Paper && self.player2 == RPSChoice::Rock)
            || (self.player1 == RPSChoice::Scissors && self.player2 == RPSChoice::Paper)
        {
            score += 6;
        }
        score
    }

    pub fn player2_score(&self) -> u32 {
        let mut score = 0;
        score += match self.player2 {
            RPSChoice::Rock => 1,
            RPSChoice::Paper => 2,
            RPSChoice::Scissors => 3,
        };
        let other_score = if self.player1 == self.player2 {
            3
        } else if (self.player1 == RPSChoice::Rock && self.player2 == RPSChoice::Scissors)
            || (self.player1 == RPSChoice::Paper && self.player2 == RPSChoice::Rock)
            || (self.player1 == RPSChoice::Scissors && self.player2 == RPSChoice::Paper)
        {
            6
        } else {
            0
        };
        score += 6 - other_score;
        score
    }
}

fn main() {
    let mut games = Vec::new();
    let mut total = 0;
    let input_file = File::open("data/day2.txt").unwrap();

    let line_reader = BufReader::new(input_file);

    for line in line_reader.lines() {
        let player1_choice = line.as_ref().unwrap().as_bytes()[0];
        let player2_choice = line.as_ref().unwrap().as_bytes()[2];
        games.push(Game {
            player1: RPSChoice::try_from(player1_choice).unwrap(),
            player2: match player2_choice {
                b'X' => RPSChoice::would_lose_from(RPSChoice::try_from(player1_choice).unwrap()),
                b'Y' => RPSChoice::try_from(player1_choice).unwrap(),
                b'Z' => RPSChoice::would_win_from(RPSChoice::try_from(player1_choice).unwrap()),
                _ => panic!(),
            },
        });
    }

    for game in games {
        total += game.player2_score();
    }

    println!("My score would be {total}.");
}
