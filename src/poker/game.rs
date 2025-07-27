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
    /// Create a new Game, and add an initial player with the given name
    pub fn new(creator_name: String) -> Game {
        let mut initial_players = Vec::new();
        initial_players.push(Player::new(creator_name));
        Game { players: initial_players, state: GameState::Waiting, button_index: 0, turn_index: 0, small_blind: DEFAULT_SMALL_BLIND, big_blind: DEFAULT_BIG_BLIND, pot: 0 }
    }

    /// Add a player to the game, will Err if the game is full. Otherwise, returns the index of the
    /// added player.
    pub fn add_player(&mut self, name: String) -> Result<usize, errors::JoinGameError> {
        let num_players = self.players.len();
        if num_players < MAX_PLAYERS {
            self.players.push(Player::new(name));
            // Since we have added a player, this will now equal the index of the added player
            return Ok(num_players);
        }
        Err(errors::JoinGameError { reason: String::from("game is full") })
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
    chips: u64,
    bet: u64,
    has_folded: bool,
    is_sitting_out: bool,
    hand: [Card; 2],
}

impl Player {
    /// Creates a new Player with the given name
    pub fn new(name: String) -> Player {
        // Initialise a new Player struct with no chips and black pocket aces for funsies :)
        Player { name: name, chips: DEFAULT_STARTING_STACK, bet: 0, has_folded: false, is_sitting_out: true, hand: [Card::new(14, Suit::Spades), Card::new(14, Suit::Clubs)] }
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
        let game = Game::new(String::from("test player"));
        assert_eq!(game.players.len(), 1, "Newly created games should have a single player");
        assert_eq!(game.players.get(0).unwrap().name, "test player", "Newly created games should contain their creator");
        assert_eq!(game.state, GameState::Waiting, "New games should begin in a waiting state");
        assert_eq!(game.button_index, 0, "New games should start with the creator as the button");
        assert_eq!(game.turn_index, 0, "New games should start with the creator as the current turn");
        assert_eq!(game.pot, 0, "New games should have no chips in the pot");
        assert_eq!(game.small_blind, DEFAULT_SMALL_BLIND, "New games should start with the default small blind");
        assert_eq!(game.big_blind, DEFAULT_BIG_BLIND, "New games should start with the default big blind");
    }

    #[test]
    fn test_add_player() {
        let mut game = Game::new(String::from("Player 1"));
        for i in 1..MAX_PLAYERS {
            assert_eq!(game.players.len(), i, "Check the number of players before we add one");
            let idx = game.add_player(format!("Player {}", i + 1));
            assert!(idx.is_ok(), "Adding a player should not create an error if the game isn't full");
            assert_eq!(idx.unwrap(), i, "The correct index should be returned when adding a new player");
            assert_eq!(game.players.len(), i + 1, "Check the number of players increased");
        }
        assert!(game.add_player(String::from("Too many")).is_err(), "Check that adding too many players creates an error");
    }

    #[test]
    fn test_start_game() {
        let mut game = Game::new(String::from("Player 1"));
        assert!(game.start().is_err(), "Test that a game with one player in cannot be started");
        assert!(game.add_player(String::from("Player 2")).is_ok(), "Test that we can add a player");
        assert!(game.start().is_ok(), "Test that we start the game once enough players have been added");
        assert!(game.start().is_err(), "Test that a game can only be started once");
    }
}