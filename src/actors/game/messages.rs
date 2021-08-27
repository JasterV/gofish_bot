use crate::{alias::Cx, command::Command};
use tokio::sync::oneshot::Sender as Responder;

#[derive(Debug)]
pub enum GameCommand {
    Start,
    Join,
    Status,
    Ask(usize, usize),
}

impl From<Command> for GameCommand {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Join => GameCommand::Join,
            Command::Start => GameCommand::Start,
            Command::Ask { to, card } => GameCommand::Ask(to, card),
            Command::Status => GameCommand::Status,
            _ => panic!("Cannot convert Command to GameCommand"),
        }
    }
}

pub struct IsOver(pub Responder<bool>);
pub struct Message(pub Cx, pub GameCommand);

pub enum GameActorMsg {
    Message(Message),
    IsOver(IsOver),
}
