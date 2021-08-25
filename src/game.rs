use crate::errors::ActionError::*;
use crate::models::deck::Deck;
use crate::models::player::{Player, PlayerState};
use anyhow::Result;
use serde::Serialize;

pub enum ActionResult {
    // Group(card)
    Group(u8),
    // Drawn(card)
    Drawn(u8),
    // Took(quantity)
    Took(u8),
    // GameOver(username)
    GameOver(String),
    // NextTurn(username)
    NextTurn(String),
}

#[derive(Serialize, Clone, Debug)]
pub struct Game {
    pub deck: Deck,
    pub game_over: bool,
    pub curr_player: usize,
    pub players: Vec<Player>,
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

    fn get_player_index(&self, player_id: &str) -> Option<usize> {
        self.players
            .iter()
            .enumerate()
            .find(|elem| elem.1.id == player_id)
            .map(|(index, _)| index)
    }

    fn pass_to_next_player(&mut self) {
        self.players[self.curr_player].state = PlayerState::None;
        self.curr_player = (self.curr_player + 1) % self.players.len();
        self.players[self.curr_player].state = PlayerState::Asking;
    }

    pub fn join_player(&mut self, player_id: String, name: &str) {
        if let None = self.get_player_index(&player_id) {
            self.players.push(Player {
                cards: vec![],
                state: PlayerState::None,
                score: 0,
                name: name.into(),
                id: player_id,
            })
        }
    }

    pub fn start_game(&mut self) {
        self.deck.shuffle();
        for player in &mut self.players {
            player.cards.extend(self.deck.draw_n(7));
        }
        self.players[self.curr_player].state = PlayerState::Asking;
    }

    pub fn take_from(
        &mut self,
        player_id: String,
        from: usize,
        card: u8,
    ) -> Result<Vec<ActionResult>> {
        let mut results = vec![];

        let index = self
            .get_player_index(&player_id)
            .ok_or_else(|| InvalidPlayerId(player_id))?;

        if self.players.len() <= from {
            return Err(WrongPlayer(from).into());
        }
        if card < 1 || card > 12 {
            return Err(WrongCard(card).into());
        }
        if self.players[index].state != PlayerState::Asking {
            return Err(CannotTake(self.players[index].name.clone()).into());
        }

        let taken = {
            let from = &mut self.players[from];
            let taken = from.remove_cards(card);
            // Check if the game is over
            if from.cards.len() == 0 {
                self.game_over = true;
            }
            taken
        };

        let player = &mut self.players[index];

        // Take cards from one player to another
        player.add_cards(&taken);
        results.push(ActionResult::Took(taken.len() as u8));
        // Set player state to drawing if no cards were taken
        if taken.len() == 0 {
            player.state = PlayerState::Drawing;
        }
        // Group player cards
        let groups = player.reduce_groups();
        for group in groups {
            results.push(ActionResult::Group(group))
        }

        if self.game_over {
            results.push(ActionResult::GameOver(self.players[from].name.clone()));
        }

        Ok(results)
    }

    pub fn draw_card(&mut self, player_id: String, chosen_card: u8) -> Result<Vec<ActionResult>> {
        let mut results = vec![];
        let index = self
            .get_player_index(&player_id)
            .ok_or_else(|| InvalidPlayerId(player_id))?;

        let player = &mut self.players[index];

        if player.state != PlayerState::Drawing {
            return Err(CannotDraw(player.name.clone()).into());
        }

        let drawn = self.deck.draw_n(1);

        player.add_cards(&drawn);
        // Check if any cards were drawn
        if let Some(&card) = drawn.first() {
            results.push(ActionResult::Drawn(card));
            let groups = player.reduce_groups();
            for card in groups {
                results.push(ActionResult::Group(card));
            }
            if card != chosen_card {
                self.pass_to_next_player();
                results.push(ActionResult::NextTurn(
                    self.players[self.curr_player].name.clone(),
                ))
            }
        } else {
            self.pass_to_next_player();
        }
        Ok(results)
    }
}
