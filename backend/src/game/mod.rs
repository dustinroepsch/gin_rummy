use serde::{Deserialize, Serialize};

use self::card::Card;

pub mod card;
pub mod deck;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Player {
    A,
    B,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlayerState {
    hand: Vec<Card>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    deck: deck::Deck,
    dealer: Player,
    current_player: Player,
    player_a: PlayerState,
    player_b: PlayerState,
}

impl Game {
    pub fn new() -> Self {
        let mut deck = deck::Deck::new();
        deck.shuffle();
        let mut player_a = PlayerState::default();
        let mut player_b = PlayerState::default();
        for _ in 0..10 {
            deck.deal_to(&mut player_a);
            deck.deal_to(&mut player_b);
        }

        Self {
            deck,
            dealer: Player::A,
            current_player: Player::A,
            player_a,
            player_b,
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
