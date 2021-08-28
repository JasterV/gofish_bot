#[macro_use]
extern crate lazy_static;

mod actors;
mod alias;
mod command;
mod entities;
mod errors;
mod templates;
mod webhook;

use crate::actors::run_async_actor;
use actors::game::{
    actor::GameActor,
    messages::{GameActorMsg, IsOver, Message},
};
use alias::Cx;
use anyhow::Result;
use command::Command;
use dashmap::DashMap;
use dotenv;
use std::sync::Arc;
use teloxide::{prelude::*, types::Me, utils::command::BotCommand};
use tokio::sync::{mpsc::Sender, oneshot};
use webhook::webhook;

lazy_static! {
    static ref SENDERS: Arc<DashMap<i64, Sender<GameActorMsg>>> = Arc::new(DashMap::new());
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    dotenv::dotenv().ok();
    teloxide::enable_logging!();
    log::info!("Starting bot...");
    let bot = Bot::from_env().auto_send();
    let Me { user: bot_user, .. } = bot.get_me().await.unwrap();
    let bot_name = bot_user.username.expect("Bots must have usernames");
    log::info!("listening...");
    let cloned_bot = bot.clone();
    teloxide::commands_repl_with_listener(bot, bot_name, execute, webhook(cloned_bot).await).await
}

async fn execute(cx: Cx, command: Command) -> Result<()> {
    let chat_id = cx.chat_id();

    match command {
        Command::Help => {
            cx.answer(Command::descriptions()).await?;
        }
        Command::NewGame => {
            SENDERS.entry(chat_id).or_insert_with(|| {
                let addr: Sender<GameActorMsg> = run_async_actor(GameActor::new());
                addr
            });
            cx.answer("Game created! Start joining and send start to start fishing")
                .await?;
        }
        Command::EndGame => {
            if let Some(_) = SENDERS.remove(&chat_id) {
                cx.answer("The game has end!").await?;
            } else {
                cx.answer("There is no game in progress").await?;
            }
        }
        _ => {
            let entry = SENDERS.get(&chat_id);
            if let Some(entry) = entry {
                let sender = entry.value();
                let _ = sender
                    .send(GameActorMsg::Message(Message(cx, command.into())))
                    .await;
                let (tx, rx) = oneshot::channel();
                let _ = sender.send(GameActorMsg::IsOver(IsOver(tx))).await;
                let is_over = rx.await?;
                if is_over {
                    SENDERS.remove(&chat_id);
                }
            } else {
                cx.answer("The game has not been created yet!").await?;
            }
        }
    }
    Ok(())
}
