use crate::alias::Cx;
use anyhow::Result;
use teloxide::prelude::*;

pub async fn say_hi(cx: &Cx) -> Result<()> {
    let bot = cx.requester.clone();
    let message = cx.update.clone();
    let sender = message.from().expect("User data not found");
    // let sender_chat = message.sender_chat().expect("Sender chat not found");
    let info = format!(
        "Your data: \n \t fullname: {} \n \t username: {} \n \t id: {}",
        sender.full_name(),
        sender.username.clone().unwrap_or("unknown".into()),
        sender.id
    );
    bot.send_message(cx.update.chat_id(), info.clone())
        .send()
        .await?;
    bot.send_message(sender.id, info).send().await?;
    Ok(())
}
