use crate::{alias::Cx, command::Command as TCommand};

pub enum Command {
    SendCmd(TCommand, Cx),
}
