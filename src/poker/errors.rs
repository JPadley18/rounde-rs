use std::fmt;

/// Error type to be returned when failing to join a game
#[derive(Debug, Clone)]
pub struct JoinGameError {
    pub kind: JoinGameErrorKind
}

#[derive(Debug, Clone)]
pub enum JoinGameErrorKind {
    GameFull,
}

impl fmt::Display for JoinGameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.kind {
            JoinGameErrorKind::GameFull => write!(f, "The game is full")
        }
    }
}

/// Error type to be returned when failing to start a game
#[derive(Debug, Clone)]
pub struct GameStartError {
    pub kind: GameStartErrorKind
}

#[derive(Debug, Clone)]
pub enum GameStartErrorKind {
    InsufficientPlayers,
    GameInProgress,
}

impl fmt::Display for GameStartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.kind {
            GameStartErrorKind::InsufficientPlayers => write!(f, "Cannot start a game with fewer than two players"),
            GameStartErrorKind::GameInProgress => write!(f, "Cannot start a game that is in progress"),
        }
    }
}