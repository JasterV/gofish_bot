use super::messages::GameCommand;
use crate::alias::Cx;
use crate::entities::actor::AsyncActor;
use crate::entities::game::Action;
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

    async fn start(&mut self, cx: Cx) -> Result<()> {
        self.game.execute(Action::Start)?;
        cx.answer(GAME_STARTED).await?;
        self.send_status_to_players(cx).await?;
        Ok(())
    }

    async fn join(&mut self, cx: Cx) -> Result<()> {
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

    async fn ask(&mut self, cx: Cx, to: usize, card: usize) -> Result<()> {
        Ok(())
    }

    async fn status(&self, cx: Cx) -> Result<()> {
        Ok(())
    }

    async fn my_status(&self, cx: Cx) -> Result<()> {
        Ok(())
    }

    async fn end(&mut self, cx: Cx) -> Result<()> {
        Ok(())
    }

    async fn send_status_to_players(&self, cx: Cx) -> Result<()> {
        let bot = &cx.requester;
        for player in &self.game.players {
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
impl AsyncActor<Message> for GameActor {
    type Output = ();

    async fn handle(&mut self, Message(cx, command): Message) -> Result<Self::Output> {
        let result = match command {
            GameCommand::Ask(to, card) => self.ask(cx, to, card).await,
            GameCommand::End => self.end(cx).await,
            GameCommand::Join => self.join(cx).await,
            GameCommand::Start => self.start(cx).await,
            GameCommand::Status => self.status(cx).await,
            GameCommand::MyStatus => self.my_status(cx).await,
        };
        // Check errors ocurred
        Ok(())
    }
}
