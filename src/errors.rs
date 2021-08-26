use thiserror::Error;

#[derive(Error, Debug)]
pub enum ActionError {
    #[error("Player with number {0} does not exist")]
    WrongPlayer(usize),
    #[error("Invalid question, check the option you chose or the card number. \n Option: {0}, Card: {1}")]
    InvalidQuestion(usize, u8),
    #[error("There is no card with the number {0}")]
    WrongCard(u8),
    #[error("There is no player with id {0}")]
    InvalidPlayerId(String),
    #[error("{0} can't ask for cards now")]
    CannotAsk(String),
    #[error("{0} can't draw cards now")]
    CannotDraw(String),
    #[error("Game already started")]
    GameAlreadyStarted,
    #[error("{0} is already in the game")]
    PlayerAlreadyJoined(String),
}
