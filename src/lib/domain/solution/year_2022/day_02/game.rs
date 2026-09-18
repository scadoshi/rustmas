use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub enum GameResult {
    Loss,
    Draw,
    Win,
}

impl GameResult {
    pub fn value(&self) -> u32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

#[derive(Debug, Error)]
#[error("invalid play")]
pub struct InvalidPlay;

#[derive(Debug, Clone, Copy)]
pub enum Play {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Play {
    type Error = InvalidPlay;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value.to_ascii_lowercase() {
            'a' | 'x' => Self::Rock,
            'b' | 'y' => Self::Paper,
            'c' | 'z' => Self::Scissors,
            _ => return Err(InvalidPlay),
        })
    }
}

impl Play {
    pub fn value(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn against(self, rhs: Play) -> GameResult {
        match (self, rhs) {
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => GameResult::Win,
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => GameResult::Loss,
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => GameResult::Draw,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Game {
    pub player: Play,
    pub opponent: Play,
}

impl Game {
    pub fn new(player: Play, opponent: Play) -> Self {
        Self { player, opponent }
    }

    pub fn result(self) -> GameResult {
        self.player.against(self.opponent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_try_from_char_ok() {
        assert!(matches!(Play::try_from('a').unwrap(), Play::Rock));
        assert!(matches!(Play::try_from('A').unwrap(), Play::Rock));
        assert!(matches!(Play::try_from('x').unwrap(), Play::Rock));
        assert!(matches!(Play::try_from('X').unwrap(), Play::Rock));

        assert!(matches!(Play::try_from('b').unwrap(), Play::Paper));
        assert!(matches!(Play::try_from('B').unwrap(), Play::Paper));
        assert!(matches!(Play::try_from('y').unwrap(), Play::Paper));
        assert!(matches!(Play::try_from('Y').unwrap(), Play::Paper));

        assert!(matches!(Play::try_from('c').unwrap(), Play::Scissors));
        assert!(matches!(Play::try_from('C').unwrap(), Play::Scissors));
        assert!(matches!(Play::try_from('z').unwrap(), Play::Scissors));
        assert!(matches!(Play::try_from('Z').unwrap(), Play::Scissors));
    }

    #[test]
    fn game_result() {
        [
            Game::new(Play::Rock, Play::Scissors),
            Game::new(Play::Paper, Play::Rock),
            Game::new(Play::Scissors, Play::Paper),
        ]
        .iter()
        .for_each(|g| assert!(matches!(g.result(), GameResult::Win)));
        [
            Game::new(Play::Rock, Play::Paper),
            Game::new(Play::Paper, Play::Scissors),
            Game::new(Play::Scissors, Play::Rock),
        ]
        .iter()
        .for_each(|g| assert!(matches!(g.result(), GameResult::Loss)));
        [
            Game::new(Play::Rock, Play::Rock),
            Game::new(Play::Paper, Play::Paper),
            Game::new(Play::Scissors, Play::Scissors),
        ]
        .iter()
        .for_each(|g| assert!(matches!(g.result(), GameResult::Draw)));
    }
}
