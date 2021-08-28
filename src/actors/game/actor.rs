use super::messages::{GameActorMsg, GameCommand, IsOver};
use crate::actors::AsyncActor;
use crate::alias::Cx;
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
                        cx.answer(INVALID_QUESTION).await?;
                    }
                    ActionError::InvalidPlayerId(_) => {
                        cx.answer(INVALID_PLAYER).await?;
                    }
                    ActionError::CannotAsk(_) => {
                        cx.answer(NOT_YOUR_TURN).await?;
                    }
                    ActionError::CannotDraw(_) => {
                        cx.answer(ERROR_DRAWING).await?;
                    }
                    ActionError::GameAlreadyStarted => {
                        cx.answer(GAME_ALREADY_STARTED).await?;
                    }
                    ActionError::PlayerAlreadyJoined(_) => {
                        cx.answer(ALREADY_JOINED).await?;
                    }
                },
                None => {
                    cx.answer(UNKNOWN_ERROR).await?;
                }
            }
        }
        Ok(())
    }
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
                        cx.answer(no_cards(&self.game.players[to].name)).await?;
                    } else {
                        cx.answer(had_n_cards(&self.game.players[to].name, quantity, card))
                            .await?;
                    }
                }
                TurnEvent::Group(card) => {
                    cx.answer(made_group(&from.first_name, card)).await?;
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
                    cx.answer(drawn_card(&from.first_name)).await?;
                    if drawn == (card as u8) {
                        cx.answer(drawn_expected_card(&from.first_name, drawn))
                            .await?;
                    }
                }
                TurnEvent::DeckEmpty => {
                    cx.answer(EMPTY_DECK).await?;
                }
                TurnEvent::Group(card) => {
                    cx.answer(made_group(&from.first_name, card)).await?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    async fn status(&self, cx: &Cx) -> Result<()> {
        cx.answer(game_status(&self.game.players, self.game.deck.len()))
            .await?;
        Ok(())
    }

    async fn check_game_state(&self, cx: &Cx) -> Result<()> {
        match &self.game.state {
            GameState::Asking(index) => {
                cx.answer(ask_for_cards(
                    &self.game.players[*index].name,
                    &self.game.players,
                ))
                .await?;
            }
            GameState::GameOver(GameResults { winners, score }) => {
                cx.answer(game_over(winners, *score)).await?;
            }
            _ => {}
        }
        Ok(())
    }

    async fn send_status_to_players(&self, cx: &Cx, players: &[Player]) -> Result<()> {
        let bot = &cx.requester;
        for player in players {
            bot.send_message(player.id.clone(), player_status(player))
                .send()
                .await?;
        }
        Ok(())
    }
}
