mod actions;
mod alias;
mod command;
mod db;
mod errors;

use crate::db::connect;
use alias::{Cx, MobcPool};
use anyhow::Result;
use command::Command;
use dotenv;
use once_cell::sync::OnceCell;
use teloxide::{prelude::*, types::Me};

pub static POOL: OnceCell<MobcPool> = OnceCell::new();

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    teloxide::enable_logging!();
    let url = std::env::var("REDIS_URL").expect("REDIS_URL not found");
    run(&url).await;
}

async fn run(redis_url: &str) {
    log::info!("Connecting to redis...");
    initialize_pool(redis_url).await;
    log::info!("Starting bot...");
    let (bot, bot_name) = initialize_bot().await;
    log::info!("listening...");
    teloxide::commands_repl(bot, bot_name, execute).await;
}

async fn initialize_bot() -> (AutoSend<Bot>, String) {
    let bot = Bot::from_env().auto_send();
    let Me { user: bot_user, .. } = bot.get_me().await.unwrap();
    let bot_name = bot_user.username.expect("Bots must have usernames");
    (bot, bot_name)
}

async fn initialize_pool(url: &str) {
    let pool = connect(url).await.expect("Error connecting to redis");
    POOL.set(pool)
        .map_err(|_| ())
        .expect("Error initializing pool");
}

async fn execute(cx: Cx, command: Command) -> Result<()> {
    let pool = POOL.get().unwrap();
    match command {
        Command::Help => actions::help(&cx).await?,
        _ => actions::say_hi(&cx).await?,
    };
    Ok(())
}
