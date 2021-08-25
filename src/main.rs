#[macro_use]
extern crate lazy_static;

mod actors;
mod alias;
mod command;
mod errors;
mod models;

use crate::actors::{
    cmd_processor::{actor::CmdProcessor, messages::Command as PxCommand},
    run_async_actor,
};
use command::Command;
use dotenv;
use teloxide::{prelude::*, types::Me};
use tokio::sync::mpsc::Sender;

lazy_static! {
    static ref ADDR: Sender<PxCommand> = run_async_actor(CmdProcessor::new());
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
    teloxide::commands_repl(bot, bot_name, |cx, cmd: Command| async move {
        ADDR.send(PxCommand::SendCmd(cmd, cx)).await.map_err(|_| ())
    })
    .await;
}
