mod actions;
mod alias;
mod command;
mod db;
mod errors;

use alias::Cx;
use anyhow::Result;
use command::Command;
use dotenv;
use teloxide::{prelude::*, types::Me};

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
    let pool = db::POOL.get().await;
    match command {
        Command::Help => actions::help(&cx).await?,
        _ => actions::say_hi(&cx).await?,
    };
    Ok(())
}
