use super::deck::Deck;
use anyhow::Result;
use serde::Serialize;

pub enum StepResponse {
    // Draw(is_success)
    Draw(bool),
    // Continue(card, quantity)
    Continue(u8, u8),
}

#[derive(Serialize, Clone, Debug)]
pub struct Player {
    pub name: String,
    pub cards: Vec<u8>,
    pub score: u8,
}

#[derive(Serialize, Clone, Debug)]
pub struct Game {
    deck: Deck,
    game_over: bool,
    curr_player: u8,
    players: Vec<Player>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            deck: Deck::new(),
            game_over: false,
            curr_player: 0,
            players: vec![],
        }
    }

    pub fn join_player(&mut self, name: &str) {}

    pub fn start_game(&mut self) {}

    pub fn ask(&mut self, from: u8, to: u8, card: u8) -> Result<u8> {
        Ok(0)
    }

    pub fn take_from(&mut self, player: u8, from: u8, card: u8) -> Result<()> {
        Ok(())
    }

    pub fn draw_card(&mut self, player: u8) -> Result<Option<u8>> {
        Ok(None)
    }

    pub fn is_over(&self) -> bool {
        self.game_over
    }
}
