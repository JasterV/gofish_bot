use crate::alias::Cx;
use crate::command::Command;
use anyhow::Result;
use teloxide::{prelude::*, utils::command::BotCommand};

pub async fn help(cx: &Cx) -> Result<()> {
    cx.answer(Command::descriptions())
        .send()
        .await
        .map(|_| ())?;
    Ok(())
}
