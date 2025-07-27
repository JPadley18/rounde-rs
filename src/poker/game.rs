use uuid::Uuid;

use crate::poker::cards::Card;
use crate::poker::cards::Suit;
use crate::poker::errors;

// Texas Hold'em constants
const DEFAULT_SMALL_BLIND: u64 = 1;
const DEFAULT_BIG_BLIND: u64 = 2;
const DEFAULT_STARTING_STACK: u64 = 250;
const MAX_PLAYERS: usize = 8;

/// A game encapsulates a whole game of Texas Hold'em poker
pub struct Game {
    players: Vec<Player>,
    state: GameState,
    button_index: usize,
    turn_index: usize,
    small_blind: u64,
    big_blind: u64,
    pot: u64,
}

impl Game {
    /// Instantiate a new Game
    pub fn new() -> Game {
        Game { players: Vec::new(), state: GameState::Waiting, button_index: 0, turn_index: 0, small_blind: DEFAULT_SMALL_BLIND, big_blind: DEFAULT_BIG_BLIND, pot: 0 }
    }

    /// Add a player to the game, will Err if the game is full. Otherwise, returns the UUID of the
    /// added player so that the caller can perform actions on the player.
    pub fn add_player(&mut self, name: String) -> Result<Uuid, errors::JoinGameError> {
        let num_players = self.players.len();
        if num_players < MAX_PLAYERS {
            let new_player = Player::new(name);
            let uuid = new_player.uuid;
            self.players.push(new_player);
            // Since we have added a player, this will now equal the index of the added player
            return Ok(uuid);
        }
        Err(errors::JoinGameError { reason: String::from("game is full") })
    }

    /// Removes a player by their UUID. Returns Err if the player doesn't exist in the game
    pub fn remove_player(&mut self, uuid: Uuid) -> Result<(), errors::LeaveGameError> {
        for (i, player) in self.players.iter().enumerate() {
            if player.uuid == uuid {
                self.players.remove(i);
                return Ok(())
            }
        }
        Err(errors::LeaveGameError { uuid })
    }

    /// Starts the game. This will error unless the game state is equal to GameState::Waiting
    pub fn start(&mut self) -> Result<(), errors::GameStartError> {
        match self.state {
            GameState::Waiting => {
                if self.players.len() > 1 {
                    // TODO: actually start the game properly
                    self.state = GameState::PreFlop;
                    return Ok(())
                }
                Err(errors::GameStartError{ reason: String::from("insufficient players to start the game") })
            },
            _ => Err(errors::GameStartError{ reason: String::from("game is already in progress") })
        }
    }
}

/// Player represents a single player in a game of Texas Hold'em
pub struct Player {
    name: String,
    uuid: Uuid,
    chips: u64,
    bet: u64,
    has_folded: bool,
    is_sitting_out: bool,
    hand: [Card; 2],
}

impl Player {
    /// Creates a new Player with the given name
    fn new(name: String) -> Player {
        // Initialise a new Player struct with no chips and black pocket aces for funsies :)
        Player { name: name, uuid: Uuid::new_v4(), chips: DEFAULT_STARTING_STACK, bet: 0, has_folded: false, is_sitting_out: true, hand: [Card::new(14, Suit::Spades), Card::new(14, Suit::Clubs)] }
    }
}

/// All of the possible states that the game could be in
#[derive(PartialEq, Debug)]
pub enum GameState {
    Waiting,
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_player() {
        let player = Player::new(String::from("test player"));
        assert_eq!(player.name, "test player", "Newly created players should use the name given to them");
        assert_eq!(player.bet, 0, "Newly created players should not have a bet");
        assert_eq!(player.chips, DEFAULT_STARTING_STACK, "New players should have the default starting stack");
        assert!(!player.has_folded, "Newly created players have not folded their hand");
        assert!(player.is_sitting_out, "Newly created players should begin sitting out until a new round starts");
    }

    #[test]
    fn test_create_game() {
        let game = Game::new();
        assert_eq!(game.players.len(), 0, "Newly created games should have no players");
        assert_eq!(game.state, GameState::Waiting, "New games should begin in a waiting state");
        assert_eq!(game.button_index, 0, "New games should start with the button on player 0");
        assert_eq!(game.turn_index, 0, "New games should start with the action on player 0");
        assert_eq!(game.pot, 0, "New games should have no chips in the pot");
        assert_eq!(game.small_blind, DEFAULT_SMALL_BLIND, "New games should start with the default small blind");
        assert_eq!(game.big_blind, DEFAULT_BIG_BLIND, "New games should start with the default big blind");
    }

    #[test]
    fn test_add_player() {
        let mut game = Game::new();
        for i in 0..MAX_PLAYERS {
            assert_eq!(game.players.len(), i, "Check the number of players before we add one");
            let idx = game.add_player(format!("Player {}", i + 1));
            assert!(idx.is_ok(), "Adding a player should not create an error if the game isn't full");
            assert_eq!(game.players.len(), i + 1, "Check the number of players increased");
        }
        assert!(game.add_player(String::from("Too many")).is_err(), "Check that adding too many players creates an error");
    }

    #[test]
    fn test_remove_player() {
        let mut game = Game::new();
        assert!(game.remove_player(Uuid::new_v4()).is_err(), "Test that removing a player from an empty game creates an error");
        let player_id = game.add_player(String::from("Player 1")).unwrap();
        assert_eq!(game.players.len(), 1, "Test that the player was really added");
        assert!(game.remove_player(player_id).is_ok(), "Test that we can remove a valid player");
        assert_eq!(game.players.len(), 0, "Test that the player really was removed");
        let second_player_id = game.add_player(String::from("Player 2")).unwrap();
        assert!(game.add_player(String::from("Player 3")).is_ok());
        assert!(game.add_player(String::from("Player 4")).is_ok());
        assert_eq!(game.players.len(), 3, "Test that we have the correct number of players before removal");
        assert!(game.players.iter().any(|player| player.uuid == second_player_id), "Test that the second player is present before removal");
        assert!(game.remove_player(player_id).is_err(), "Test that removing a non-existent player does nothing");
        assert_eq!(game.players.len(), 3, "Test that the invalid remove did not mutate the players vector");
        assert!(game.remove_player(second_player_id).is_ok(), "Test that removing a player with multiple players present removes the player");
        assert_eq!(game.players.len(), 2, "Test that we are left with the correct number of players");
        assert!(!game.players.iter().any(|player| player.uuid == second_player_id), "Test that the correct player was removed");
    }

    #[test]
    fn test_start_game() {
        let mut game = Game::new();
        assert!(game.start().is_err(), "Test that an empty game cannot be started");
        assert!(game.add_player(String::from("Player 1")).is_ok(), "Test that we can add a player");
        assert!(game.start().is_err(), "Test that a game with one player in cannot be started");
        assert!(game.add_player(String::from("Player 2")).is_ok(), "Test that we can add a player");
        assert!(game.start().is_ok(), "Test that we start the game once enough players have been added");
        assert!(game.start().is_err(), "Test that a game can only be started once");
    }
}