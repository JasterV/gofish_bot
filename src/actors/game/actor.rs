use super::messages::{GameActorMsg, GameCommand, IsOver};
use crate::alias::Cx;
use crate::entities::actor::AsyncActor;
use crate::entities::game::{Action, GameResults, GameState, TurnEvent};
use crate::entities::player::Player;
use crate::errors::ActionError;
use crate::templates::*;
use crate::{actors::game::messages::Message, entities::game::Game};
use anyhow::Result;
use async_trait::async_trait;
use teloxide::prelude::*;

pub struct GameActor {
    game: Game,
}

impl GameActor {
    pub fn new() -> Self {
        Self { game: Game::new() }
    }

    pub fn is_over(&self) -> bool {
        if let GameState::GameOver(_) = self.game.state {
            true
        } else {
            false
        }
    }

    async fn start(&mut self, cx: &Cx) -> Result<()> {
        self.game.execute(Action::Start)?;
        cx.answer_dice().await?;
        cx.answer(GAME_STARTED).await?;
        self.send_status_to_players(&cx, &self.game.players).await?;
        self.check_game_state(&cx).await
    }

    async fn join(&mut self, cx: &Cx) -> Result<()> {
        let user = cx.update.from().unwrap();

        self.game.execute(Action::Join(
            user.id.clone().to_string(),
            user.first_name.clone(),
        ))?;

        cx.answer(format!(
            "Hi {}, welcome to Go Fish!",
            user.first_name.clone()
        ))
        .await?;

        Ok(())
    }

    async fn ask(&mut self, cx: &Cx, to: usize, card: usize) -> Result<()> {
        let from = cx.update.from().unwrap();
        let events = self
            .game
            .execute(Action::Ask(format!("{}", from.id), to, card as u8))?;
        for event in events {
            match event {
                TurnEvent::Took(quantity) => {
                    if quantity == 0 {
                        cx.answer(format!(
                            "{} had no cards with that number, lets draw!",
                            &self.game.players[to].name
                        ))
                        .await?;
                    } else {
                        cx.answer(format!(
                            "{} had {} cards with the number {}, keep asking!",
                            &self.game.players[to].name, quantity, card
                        ))
                        .await?;
                    }
                }
                TurnEvent::Group(card) => {
                    cx.answer(format!(
                        "{} has made a group of four {}",
                        from.first_name, card
                    ))
                    .await?;
                }
                _ => {}
            }
        }
        if let GameState::Drawing(_) = self.game.state {
            self.draw(cx, card).await?;
        }
        self.send_status_to_players(&cx, &self.game.players).await?;
        self.check_game_state(&cx).await
    }

    async fn draw(&mut self, cx: &Cx, card: usize) -> Result<()> {
        let from = cx.update.from().unwrap();
        let events = self
            .game
            .execute(Action::Draw(format!("{}", from.id), card as u8))?;
        for event in events {
            match event {
                TurnEvent::Drawn(drawn) => {
                    cx.answer(format!("{} has drawn a card", &from.first_name))
                        .await?;
                    if drawn == (card as u8) {
                        cx.answer(format!(
                            "{} has drawn a {}!! Keep asking!",
                            &from.first_name, card
                        ))
                        .await?;
                    }
                }
                TurnEvent::DeckEmpty => {
                    cx.answer("The deck is empty!!!").await?;
                }
                TurnEvent::Group(card) => {
                    cx.answer(format!(
                        "{} has made a group of four {}",
                        from.first_name, card
                    ))
                    .await?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    async fn status(&self, cx: &Cx) -> Result<()> {
        let players_data: Vec<(String, u8, usize)> = self
            .game
            .players
            .iter()
            .map(|p| (p.name.clone(), p.score, p.cards.len()))
            .collect();
        cx.answer(format!(
            "GAME STATUS:\n\nPlayers info:\n\n{}\nDeck remaining cards: {}",
            players_data
                .iter()
                .map(|(name, score, cards_len)| format!(
                    "{} => Score: {}, Cards: {}",
                    name, score, cards_len
                ))
                .collect::<Vec<String>>()
                .join("\n\t"),
            self.game.deck.len()
        ))
        .await?;
        Ok(())
    }

    async fn check_game_state(&self, cx: &Cx) -> Result<()> {
        match &self.game.state {
            GameState::Waiting => Ok(()),
            GameState::Drawing(_) => Ok(()),
            GameState::Asking(index) => {
                self.send_asking_data(&cx, &self.game.players[*index].name)
                    .await
            }
            GameState::GameOver(GameResults { winners, score }) => {
                self.send_game_over(&cx, winners, *score).await
            }
        }
    }

    async fn send_game_over(&self, cx: &Cx, winners: &[String], score: u8) -> Result<()> {
        cx.answer(format!(
            "Game Over!\n\tWinners 👑: {}\n\tScore: {}",
            winners.join(", "),
            score
        ))
        .await?;
        Ok(())
    }

    async fn send_asking_data(&self, cx: &Cx, name: &str) -> Result<()> {
        let players_data: Vec<(usize, String)> = self
            .game
            .players
            .iter()
            .enumerate()
            .map(|(index, player)| (index, player.name.clone()))
            .collect();
        cx.answer(
            format!(
                "{} lets ask someone for a card😇:\n\nType '/ask <option> <card> with one of the following options:\n\n{}'",
                name,
                players_data.iter().map(|(index, name)| format!("{}) {}", index, name)).collect::<Vec<String>>().join("\n")
            )
        ).await?;
        Ok(())
    }

    async fn send_status_to_players(&self, cx: &Cx, players: &[Player]) -> Result<()> {
        let bot = &cx.requester;
        for player in players {
            bot.send_message(
                player.id.clone(),
                format!(
                    "Hi {0}! Here is your status 😃:\n\tCards: {1}\n\tscore: {2}",
                    player.name.clone(),
                    player
                        .cards
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    player.score
                ),
            )
            .send()
            .await?;
        }
        Ok(())
    }
}

#[async_trait]
impl AsyncActor<GameActorMsg> for GameActor {
    type Output = ();

    async fn handle(&mut self, msg: GameActorMsg) -> Result<Self::Output> {
        match msg {
            GameActorMsg::Message(msg) => self.handle_message(msg).await,
            GameActorMsg::IsOver(msg) => self.handle_is_over(msg).await,
        }
    }
}

impl GameActor {
    async fn handle_is_over(&self, IsOver(responder): IsOver) -> Result<()> {
        let _ = responder.send(self.is_over());
        Ok(())
    }

    async fn handle_message(&mut self, Message(cx, command): Message) -> Result<()> {
        let result = match command {
            GameCommand::Ask(to, card) => self.ask(&cx, to, card).await,
            GameCommand::Join => self.join(&cx).await,
            GameCommand::Start => self.start(&cx).await,
            GameCommand::Status => self.status(&cx).await,
        };

        if let Err(root_err) = result {
            match root_err.downcast_ref::<ActionError>() {
                Some(err) => match err {
                    ActionError::InvalidQuestion(_, _) => {
                        cx.answer("Invalid question! Check if the option and the card provided are correct").await?;
                    }
                    ActionError::InvalidPlayerId(_) => {
                        cx.answer("Sorry you can't ask you are not playing!")
                            .await?;
                    }
                    ActionError::CannotAsk(_) => {
                        cx.answer("Hey is not your turn! You can't ask!").await?;
                    }
                    ActionError::CannotDraw(_) => {
                        cx.answer("Error drawing :(").await?;
                    }
                    ActionError::GameAlreadyStarted => {
                        cx.answer("The game has already started!").await?;
                    }
                    ActionError::PlayerAlreadyJoined(_) => {
                        cx.answer("You have already joined!").await?;
                    }
                },
                None => {}
            }
        }
        Ok(())
    }
}
