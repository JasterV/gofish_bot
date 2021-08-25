use tokio::sync::oneshot::Sender as Responder;

#[derive(Debug)]
pub enum Command {
    Start,
    Join(String),
    // from, to, card, response
    Ask(u8, u8, u8, Responder<String>),
    // [(index, name)]
    Players(Responder<Vec<(u8, String)>>),
    Stop(Responder<(String, u8)>),
}
