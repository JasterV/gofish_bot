use crate::entities::{deck::Deck, player::Player};
use crate::errors::ActionError::*;
use anyhow::Result;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Serialize;

pub enum TurnEvents {
    Started,
    Joined,
    // Group(card)
    Group(u8),
    // Drawn(card)
    Drawn(u8),
    // Took(quantity)
    Took(u8),
    // GameOver(winners, score)
    GameOver(Vec<String>, u8),
    DeckEmpty,
    // NextTurn(username)
    NextTurn(String),
}

pub enum Action {
    Start,
    // Join(id, name)
    Join(String, String),
    // Take(player, to, card)
    Ask(String, usize, u8),
    // Draw(player, last_card)
    Draw(String, u8),
}

#[derive(Serialize, PartialEq, Clone, Debug)]
pub enum GameState {
    Waiting,
    Asking(usize),
    Drawing(usize),
}

#[derive(Serialize, Debug)]
pub struct Game {
    pub deck: Deck,
    pub state: GameState,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            deck: Deck::new(),
            state: GameState::Waiting,
            players: vec![],
        }
    }

    pub fn execute(&mut self, action: Action) -> Result<Vec<TurnEvents>> {
        let events = match action {
            Action::Start => self.start_game()?,
            Action::Join(id, name) => self.join_player(&id, &name)?,
            Action::Ask(id, to, card) => self.ask_to(id, to, card)?,
            Action::Draw(id, last_card) => self.draw_card(id, last_card)?,
        };
        Ok(events)
    }

    fn join_player(&mut self, player_id: &str, name: &str) -> Result<Vec<TurnEvents>> {
        if self.has_started() {
            return Err(GameAlreadyStarted.into());
        }
        if let Some(index) = self.get_player_index(player_id) {
            let player_name = self.players[index].name.clone();
            return Err(PlayerAlreadyJoined(player_name).into());
        }
        self.players.push(Player {
            cards: vec![],
            score: 0,
            name: name.into(),
            id: player_id.into(),
        });
        Ok(vec![TurnEvents::Joined])
    }

    fn start_game(&mut self) -> Result<Vec<TurnEvents>> {
        if self.has_started() {
            return Err(GameAlreadyStarted.into());
        }
        self.deck.shuffle();
        self.shuffle_players();
        for player in &mut self.players {
            player.cards.extend(self.deck.draw_n(7));
        }
        self.state = GameState::Asking(0);
        return Ok(vec![TurnEvents::Started]);
    }

    pub fn ask_to(&mut self, player_id: String, to: usize, card: u8) -> Result<Vec<TurnEvents>> {
        let mut events = vec![];
        let index = self
            .get_player_index(&player_id)
            .ok_or_else(|| InvalidPlayerId(player_id.clone()))?;
        if !self.can_ask(index) {
            return Err(CannotAsk(player_id.clone()).into());
        }
        if !self.is_valid_question(to, card) {
            return Err(InvalidQuestion(to, card).into());
        }
        let cards = self.take_cards_from(to, card);
        events.push(TurnEvents::Took(cards.len() as u8));
        let player = &mut self.players[index];
        // Set player state to drawing if no cards were taken
        if cards.len() > 0 {
            player.add_cards(&cards);
            let groups = player.reduce_groups();
            // Group player cards
            for group in groups {
                events.push(TurnEvents::Group(group))
            }
        } else {
            self.state = GameState::Drawing(index);
        }

        if !player.has_cards() || !self.players[to].has_cards() {
            let winners = self.get_winners();
            events.push(TurnEvents::GameOver(
                winners.iter().map(|p| p.name.clone()).collect(),
                winners[0].score,
            ));
            self.end_game();
        }

        Ok(events)
    }

    pub fn draw_card(&mut self, player_id: String, chosen_card: u8) -> Result<Vec<TurnEvents>> {
        let mut events = vec![];
        let index = self
            .get_player_index(&player_id)
            .ok_or_else(|| InvalidPlayerId(player_id.clone()))?;
        if !self.can_draw(index) {
            return Err(CannotDraw(player_id.clone()).into());
        }
        let player = &mut self.players[index];
        let drawn = self.deck.draw_n(1);
        if drawn.len() == 0 {
            events.push(TurnEvents::DeckEmpty);
        }

        if let Some(&card) = drawn.first() {
            player.add_cards(&drawn);
            events.push(TurnEvents::Drawn(card));
            if card != chosen_card {
                self.end_turn();
                events.push(TurnEvents::NextTurn(
                    self.get_curr_player().unwrap().name.clone(),
                ))
            } else {
                let groups = player.reduce_groups();
                for card in groups {
                    events.push(TurnEvents::Group(card));
                }
                if !player.has_cards() {
                    let winners = self.get_winners();
                    events.push(TurnEvents::GameOver(
                        winners.iter().map(|p| p.name.clone()).collect(),
                        winners[0].score,
                    ));
                    self.end_game();
                }
            }
        } else {
            self.end_turn();
            events.push(TurnEvents::NextTurn(
                self.get_curr_player().unwrap().name.clone(),
            ))
        }

        Ok(events)
    }

    fn end_turn(&mut self) {
        let index = match self.state {
            GameState::Drawing(index) => index,
            _ => panic!("Cannot end the turn"),
        };
        let new_index = (index + 1) % self.players.len();
        self.state = GameState::Asking(new_index);
    }

    fn end_game(&mut self) {
        self.deck = Deck::new();
        self.state = GameState::Waiting;
        self.players = vec![]
    }

    fn has_started(&self) -> bool {
        self.state != GameState::Waiting
    }

    fn get_winners(&self) -> Vec<&Player> {
        let player = self.players.iter().max_by_key(|p| p.score).unwrap();
        self.players
            .iter()
            .filter(|p| p.score == player.score)
            .collect()
    }

    fn take_cards_from(&mut self, from: usize, card: u8) -> Vec<u8> {
        let from = &mut self.players[from];
        let taken = from.remove_cards(card);
        taken
    }

    fn get_curr_player(&self) -> Option<&Player> {
        match self.state {
            GameState::Drawing(index) => Some(&self.players[index]),
            GameState::Asking(index) => Some(&self.players[index]),
            _ => None,
        }
    }

    fn can_draw(&self, index: usize) -> bool {
        self.state == GameState::Asking(index)
    }

    fn can_ask(&self, index: usize) -> bool {
        self.state == GameState::Drawing(index)
    }

    fn is_valid_question(&self, to: usize, card: u8) -> bool {
        self.is_valid_player_index(to) && Deck::valid_card(card)
    }

    fn is_valid_player_index(&self, to: usize) -> bool {
        to < self.players.len()
    }

    fn shuffle_players(&mut self) {
        let mut rng = thread_rng();
        self.players.shuffle(&mut rng);
    }

    fn get_player_index(&self, player_id: &str) -> Option<usize> {
        self.players
            .iter()
            .enumerate()
            .find(|elem| elem.1.id == player_id)
            .map(|(index, _)| index)
    }
}
