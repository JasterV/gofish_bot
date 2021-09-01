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
            let message = match root_err.downcast_ref::<ActionError>() {
                Some(err) => match err {
                    ActionError::InvalidQuestion(_, _) => INVALID_QUESTION,
                    ActionError::InvalidPlayerId(_) => INVALID_PLAYER,
                    ActionError::CannotAsk(_) => NOT_YOUR_TURN,
                    ActionError::CannotDraw(_) => ERROR_DRAWING,
                    ActionError::GameAlreadyStarted => GAME_ALREADY_STARTED,
                    ActionError::PlayerAlreadyJoined(_) => ALREADY_JOINED,
                },
                None => UNKNOWN_ERROR,
            };
            cx.answer(message).await?;
        }
        Ok(())
    }
}

impl GameActor {
    pub fn new() -> Self {
        Self { game: Game::new() }
    }

    pub fn is_over(&self) -> bool {
        match self.game.state {
            GameState::GameOver(_) => true,
            _ => false,
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
        cx.answer(welcome(&user.first_name)).await?;
        Ok(())
    }

    async fn ask(&mut self, cx: &Cx, to: usize, card: usize) -> Result<()> {
        let from = cx.update.from().unwrap();
        let player_id = format!("{}", from.id);
        let events = self
            .game
            .execute(Action::Ask(player_id.clone(), to, card as u8))?;
        for event in events {
            let msg = match event {
                TurnEvent::Took(quantity) if quantity == 0 => {
                    Some(no_cards(&self.game.players[to].name))
                }
                TurnEvent::Took(quantity) => {
                    Some(had_n_cards(&self.game.players[to].name, quantity, card))
                }
                TurnEvent::Group(card) => Some(made_group(&from.first_name, card)),
                _ => None,
            };
            if let Some(msg) = msg {
                cx.answer(msg).await?;
            }
        }
        if let GameState::Drawing(_) = self.game.state {
            self.draw(cx, card).await?;
        }
        self.send_status_to_players(
            &cx,
            &vec![
                self.game.players[to].clone(),
                self.game.get_player_by_id(&player_id).unwrap().clone(),
            ],
        )
        .await?;
        self.check_game_state(&cx).await
    }

    async fn draw(&mut self, cx: &Cx, card: usize) -> Result<()> {
        let from = cx.update.from().unwrap();
        let events = self
            .game
            .execute(Action::Draw(format!("{}", from.id), card as u8))?;
        for event in events {
            let msg = match event {
                TurnEvent::Drawn(drawn) if drawn == (card as u8) => {
                    Some(drawn_expected_card(&from.first_name, drawn))
                }
                TurnEvent::Drawn(_) => Some(drawn_card(&from.first_name)),
                TurnEvent::DeckEmpty => Some(EMPTY_DECK.into()),
                TurnEvent::Group(card) => Some(made_group(&from.first_name, card)),
                _ => None,
            };
            if let Some(msg) = msg {
                cx.answer(msg).await?;
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
