use crate::{alias::Cx, command::Command};

#[derive(Debug)]
pub enum GameCommand {
    Start,
    Join,
    Status,
    Ask(usize, usize),
    End,
}

impl From<Command> for GameCommand {
    fn from(cmd: Command) -> Self {
        match cmd {
            Command::Join => GameCommand::Join,
            Command::Start => GameCommand::Start,
            Command::EndGame => GameCommand::End,
            Command::Ask { to, card } => GameCommand::Ask(to, card),
            Command::Status => GameCommand::Status,
            _ => panic!("Cannot convert Command to GameCommand"),
        }
    }
}

pub struct Message(pub Cx, pub GameCommand);
