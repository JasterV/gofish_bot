use teloxide::{adaptors::AutoSend, prelude::*, Bot};

pub type Cx = UpdateWithCx<AutoSend<Bot>, Message>;
