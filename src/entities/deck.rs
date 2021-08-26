use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Deck {
    cards: Vec<u8>,
}

impl Deck {
    pub fn new() -> Self {
        Self {
            cards: vec![
                1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7,
                8, 8, 8, 8, 9, 9, 9, 9, 10, 10, 10, 10, 11, 11, 11, 11, 12, 12, 12, 12,
            ],
        }
    }

    pub fn valid_card(card: u8) -> bool {
        card >= 1 && card <= 12
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw_n(&mut self, n: usize) -> Vec<u8> {
        let len = self.cards.len();
        self.cards.drain((len - n)..).collect()
    }
}
