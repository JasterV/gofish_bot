#[macro_use]
extern crate lazy_static;

mod actors;
mod alias;
mod command;
mod entities;
mod errors;

use std::sync::Arc;

use crate::actors::run_async_actor;
use actors::game::{actor::GameActor, messages::Message};
use alias::Cx;
use anyhow::Result;
use command::Command;
use dashmap::DashMap;
use dotenv;
use teloxide::{prelude::*, types::Me};
use tokio::sync::mpsc::Sender;

lazy_static! {
    static ref SENDERS: Arc<DashMap<i64, Sender<Message>>> = Arc::new(DashMap::new());
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    teloxide::enable_logging!();
    run().await;
}

async fn run() {
    log::info!("Starting bot...");
    let bot = Bot::from_env().auto_send();
    let Me { user: bot_user, .. } = bot.get_me().await.unwrap();
    let bot_name = bot_user.username.expect("Bots must have usernames");

    log::info!("listening...");
    teloxide::commands_repl(bot, bot_name, execute).await;
}

async fn execute(cx: Cx, command: Command) -> Result<()> {
    let chat_id = cx.chat_id();

    match command {
        Command::NewGame => {
            SENDERS.entry(chat_id).or_insert_with(|| {
                let addr: Sender<Message> = run_async_actor(GameActor::new());
                addr
            });
        }
        Command::EndGame => {
            let entry = SENDERS.remove(&chat_id);
            if let Some(entry) = entry {
                let _ = entry.1.send(Message(cx, command.into())).await;
            }
        }
        _ => {
            let entry = SENDERS.get(&chat_id);
            if let Some(entry) = entry {
                let _ = entry.value().send(Message(cx, command.into())).await;
            } else {
                cx.answer("The game has not started yet!");
            }
        }
    }
    Ok(())
}
