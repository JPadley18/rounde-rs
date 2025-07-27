use snafu::prelude::*;
use uuid::Uuid;

/// Error type to be returned when failing to join a game
#[derive(Debug, Snafu)]
#[snafu(display("Cannot join game: {reason}"))]
pub struct JoinGameError {
    pub reason: String
}

/// Error type to be returned when trying to remove a player from a game
#[derive(Debug, Snafu)]
#[snafu(display("Player {uuid} does not exist, so cannot be removed from the game"))]
pub struct LeaveGameError {
    pub uuid: Uuid
}

/// Error type to be returned when failing to join a game
#[derive(Debug, Snafu)]
#[snafu(display("Cannot start game: {reason}"))]
pub struct GameStartError {
    pub reason: String
}