use snafu::prelude::*;

/// Error type to be returned when failing to join a game
#[derive(Debug, Snafu)]
#[snafu(display("Cannot join game: {reason}"))]
pub struct JoinGameError {
    pub reason: String
}

/// Error type to be returned when failing to join a game
#[derive(Debug, Snafu)]
#[snafu(display("Cannot start game: {reason}"))]
pub struct GameStartError {
    pub reason: String
}