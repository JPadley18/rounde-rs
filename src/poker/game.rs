use std::fmt;

use crate::poker::cards::Card;
use crate::poker::cards::Suit;

// Texas Hold'em constants
const DEFAULT_SMALL_BLIND: u64 = 1;
const DEFAULT_BIG_BLIND: u64 = 2;
const DEFAULT_STARTING_STACK: u64 = 250;
const MAX_PLAYERS: usize = 8;

#[derive(Debug, Clone)]
pub struct GameFull;

impl fmt::Display for GameFull {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "The game is full")
    }
}

// A game encapsulates a whole game of Texas Hold'em poker
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
    pub fn new(creator_name: String) -> Game {
        let mut initial_players = Vec::new();
        initial_players.push(Player::new(creator_name));
        Game { players: initial_players, state: GameState::Waiting, button_index: 0, turn_index: 0, small_blind: DEFAULT_SMALL_BLIND, big_blind: DEFAULT_BIG_BLIND, pot: 0 }
    }

    pub fn add_player(&mut self, name: String) -> Result<usize, GameFull> {
        let num_players = self.players.len();
        if num_players < MAX_PLAYERS {
            self.players.push(Player::new(name));
            // Since we have added a player, this will now equal the index of the added player
            return Ok(num_players);
        }
        Err(GameFull)
    }
}

pub struct Player {
    name: String,
    chips: u64,
    bet: u64,
    has_folded: bool,
    is_sitting_out: bool,
    hand: [Card; 2],
}

impl Player {
    pub fn new(name: String) -> Player {
        // Initialise a new Player struct with no chips and black pocket aces for funsies :)
        Player { name: name, chips: DEFAULT_STARTING_STACK, bet: 0, has_folded: false, is_sitting_out: true, hand: [Card::new(14, Suit::Spades), Card::new(14, Suit::Clubs)] }
    }
}

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
            assert!(game.add_player(format!("Player {}", i + 1)).is_ok(), "We should be able to add a player");
            assert_eq!(game.players.len(), i + 1, "Check the number of players increased");
        }
        assert!(game.add_player(String::from("Too many")).is_err(), "Check that adding too many players creates an error");
        println!("{:?}", game.add_player(String::from("Player too many")));
    }
}