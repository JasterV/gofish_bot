use super::messages::Command;
use crate::alias::Cx;
use crate::command::Command as TCommand;
use crate::{
    actors::{
        game::{actor::GameActor, messages::Command as GameCommand},
        run_actor,
    },
    models::actor::AsyncActor,
};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::mpsc::Sender;

pub struct CmdProcessor {
    games: HashMap<u16, Sender<GameCommand>>,
}

impl CmdProcessor {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
        }
    }

    async fn execute(&mut self, cmd: TCommand, cx: Cx) {}

    // async fn send_tx(&mut self, transaction: Transaction) -> Result<()> {
    //     let addr = self.accounts.entry(transaction.client).or_insert_with(|| {
    //         let actor = AccountActor::new(transaction.client);
    //         run_actor(actor)
    //     });
    //     addr.send(transaction.into()).await?;
    //     Ok(())
    // }
}

#[async_trait]
impl AsyncActor<Command> for CmdProcessor {
    type Output = ();

    async fn handle(&mut self, command: Command) -> Result<Self::Output> {
        match command {
            Command::SendCmd(cmd, cx) => self.execute(cmd, cx).await,
        }
        Ok(())
    }
}
