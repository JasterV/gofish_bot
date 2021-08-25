#[macro_use]
extern crate lazy_static;

mod actors;
mod alias;
mod command;
mod errors;
mod game;
mod models;

use std::sync::Arc;

use actors::game::messages::Message;
use alias::Cx;
use anyhow::Result;
use command::Command;
use dashmap::DashMap;
use dotenv;
use lazy_static::lazy_static;
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
        Command::NewGame => {}
        Command::EndGame => {}
        _ => {}
    }
    Ok(())
}
