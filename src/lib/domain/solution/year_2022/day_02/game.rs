use thiserror::Error;

#[derive(Debug, Error)]
#[error("invalid play")]
pub struct InvalidPlay;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// Every play, so a search over them covers the rules exactly once.
    const ALL: [Self; 3] = [Self::Rock, Self::Paper, Self::Scissors];

    pub fn value(self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    /// How this play fares against `other`.
    pub fn against(self, other: Play) -> GameResult {
        match (self, other) {
            (Self::Rock, Self::Paper)
            | (Self::Paper, Self::Scissors)
            | (Self::Scissors, Self::Rock) => GameResult::Loss,
            (Self::Rock, Self::Rock)
            | (Self::Paper, Self::Paper)
            | (Self::Scissors, Self::Scissors) => GameResult::Draw,
            (Self::Rock, Self::Scissors)
            | (Self::Paper, Self::Rock)
            | (Self::Scissors, Self::Paper) => GameResult::Win,
        }
    }
}

#[derive(Debug, Error)]
#[error("invalid game result")]
pub struct InvalidGameResult;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameResult {
    Loss,
    Draw,
    Win,
}

impl TryFrom<char> for GameResult {
    type Error = InvalidGameResult;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value.to_ascii_lowercase() {
            'x' => Self::Loss,
            'y' => Self::Draw,
            'z' => Self::Win,
            _ => return Err(InvalidGameResult),
        })
    }
}

impl GameResult {
    pub fn value(self) -> u32 {
        match self {
            Self::Loss => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }

    /// The play that ends this way against `other`.
    ///
    /// Searches [`Play::against`] rather than restating the rules, so the two
    /// directions cannot disagree. Exactly one play matches, hence the `expect`.
    pub fn against(self, other: Play) -> Play {
        Play::ALL
            .into_iter()
            .find(|play| play.against(other) == self)
            .expect("one play produces every result")
    }
}

#[derive(Debug, Error)]
pub enum InvalidGame {
    #[error("expected two columns separated by whitespace")]
    MissingSeparator,
    #[error("expected one character in the first column")]
    MalformedOpponent,
    #[error("expected one character in the second column")]
    MalformedOther,
    #[error(transparent)]
    Opponent(#[from] InvalidPlay),
}

/// A line read only as far as it can be without knowing which puzzle part it is.
///
/// The first column is always the opponent's play. The second is a letter whose
/// meaning changes between the parts, so it stays a `char` until asked for.
#[derive(Debug, Clone, Copy)]
pub struct RawGame {
    opponent: Play,
    other: char,
}

impl TryFrom<&str> for RawGame {
    type Error = InvalidGame;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (opponent, other) = value
            .split_once(char::is_whitespace)
            .ok_or(InvalidGame::MissingSeparator)?;
        let mut others = other.trim().chars();
        let (Some(other), None) = (others.next(), others.next()) else {
            return Err(InvalidGame::MalformedOther);
        };
        let mut opponents = opponent.trim().chars();
        let (Some(opponent), None) = (opponents.next(), opponents.next()) else {
            return Err(InvalidGame::MalformedOpponent);
        };
        Ok(Self {
            opponent: Play::try_from(opponent)?,
            other,
        })
    }
}

impl RawGame {
    /// Reads the second column as the play to make.
    pub fn to_game_other_is_player(self) -> Result<Game, InvalidPlay> {
        let player = Play::try_from(self.other)?;
        Ok(Game {
            player,
            result: player.against(self.opponent),
        })
    }

    /// Reads the second column as the result to aim for.
    pub fn to_game_other_is_result(self) -> Result<Game, InvalidGameResult> {
        let result = GameResult::try_from(self.other)?;
        Ok(Game {
            player: result.against(self.opponent),
            result,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Game {
    player: Play,
    result: GameResult,
}

impl Game {
    pub fn player_score(self) -> u32 {
        self.player.value().saturating_add(self.result.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play_try_from_char_ok() {
        assert_eq!(Play::try_from('a').unwrap(), Play::Rock);
        assert_eq!(Play::try_from('X').unwrap(), Play::Rock);
        assert_eq!(Play::try_from('b').unwrap(), Play::Paper);
        assert_eq!(Play::try_from('Y').unwrap(), Play::Paper);
        assert_eq!(Play::try_from('c').unwrap(), Play::Scissors);
        assert_eq!(Play::try_from('Z').unwrap(), Play::Scissors);
        assert!(Play::try_from('f').is_err());
    }

    #[test]
    fn game_result_try_from_char_ok() {
        assert_eq!(GameResult::try_from('x').unwrap(), GameResult::Loss);
        assert_eq!(GameResult::try_from('Y').unwrap(), GameResult::Draw);
        assert_eq!(GameResult::try_from('z').unwrap(), GameResult::Win);
        assert!(GameResult::try_from('a').is_err());
    }

    #[test]
    fn play_against_is_the_usual_rules() {
        assert_eq!(Play::Rock.against(Play::Scissors), GameResult::Win);
        assert_eq!(Play::Rock.against(Play::Paper), GameResult::Loss);
        assert_eq!(Play::Rock.against(Play::Rock), GameResult::Draw);
    }

    /// The two directions search the same table, so they have to round trip.
    #[test]
    fn game_result_against_inverts_play_against() {
        for opponent in Play::ALL {
            for result in [GameResult::Loss, GameResult::Draw, GameResult::Win] {
                assert_eq!(result.against(opponent).against(opponent), result);
            }
        }
    }

    #[test]
    fn raw_game_keeps_the_second_column_unread() {
        let raw = RawGame::try_from("A Y").unwrap();
        assert_eq!(raw.opponent, Play::Rock);
        assert_eq!(raw.other, 'Y');
    }

    #[test]
    fn raw_game_try_from_str_err() {
        assert!(matches!(
            RawGame::try_from("AY"),
            Err(InvalidGame::MissingSeparator)
        ));
        assert!(matches!(
            RawGame::try_from("AA Y"),
            Err(InvalidGame::MalformedOpponent)
        ));
        assert!(matches!(
            RawGame::try_from("A YY"),
            Err(InvalidGame::MalformedOther)
        ));
        assert!(matches!(
            RawGame::try_from("F Y"),
            Err(InvalidGame::Opponent(_))
        ));
    }

    /// The worked example from the puzzle, read both ways.
    #[test]
    fn the_example_scores_15_then_12() {
        let raws: Vec<RawGame> = ["A Y", "B X", "C Z"]
            .iter()
            .map(|l| RawGame::try_from(*l).unwrap())
            .collect();
        let as_player: u32 = raws
            .iter()
            .map(|r| r.to_game_other_is_player().unwrap().player_score())
            .sum();
        let as_result: u32 = raws
            .iter()
            .map(|r| r.to_game_other_is_result().unwrap().player_score())
            .sum();
        assert_eq!(as_player, 15);
        assert_eq!(as_result, 12);
    }
}
