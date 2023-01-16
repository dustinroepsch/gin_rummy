use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::card::{Card, ALL_RANKS, ALL_SUITS};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for card in &self.cards {
            write!(f, " {card} ")?;
        }
        write!(f, "]")
    }
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit in ALL_SUITS.iter() {
            for rank in ALL_RANKS.iter() {
                cards.push(Card {
                    suit: *suit,
                    rank: *rank,
                });
            }
        }
        Self { cards }
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}
