use crate::entities::player::Player;

pub const GAME_STARTED: &'static str = "Game started, GO FISH! 🧜‍♀️";
pub const GAME_FINISHED: &'static str = "The game has finished!";
pub const NO_GAME_IN_PROGRESS: &'static str = "There is no game in progress";
pub const NO_GAME_CREATED: &'static str = "The game has not been created yet!";
pub const INVALID_QUESTION: &'static str =
    "Invalid question! Check if the option and the card provided are correct";
pub const INVALID_PLAYER: &'static str = "Sorry you can't ask you are not playing!";
pub const GAME_CREATED: &'static str =
    "Game created! Start joining and send start to start fishing";
pub const NOT_YOUR_TURN: &'static str = "Hey is not your turn! You can't ask!";
pub const ERROR_DRAWING: &'static str = "Error drawing :(";
pub const GAME_ALREADY_STARTED: &'static str = "The game has already started!";
pub const ALREADY_JOINED: &'static str = "You have already joined!";
pub const EMPTY_DECK: &'static str = "The deck is empty!!!";
pub const UNKNOWN_ERROR: &'static str = "An error sending a message occurred!\n\nMake sure that all game participants have started the bot on their private chats to receive your cards!!\n\nOtherwise, open an issue to: https://github.com/JasterV/gofish_bot";

pub fn no_cards(name: &str) -> String {
    format!("{} had no cards with that number, lets draw!", name)
}

pub fn had_n_cards(name: &str, quantity: u8, card: usize) -> String {
    format!(
        "{} had {} cards with the number {}, keep asking!",
        name, quantity, card
    )
}

pub fn made_group(name: &str, card: u8) -> String {
    format!("{} has made a group of four {}", name, card)
}

pub fn drawn_card(name: &str) -> String {
    format!("{} has drawn a card", name)
}

pub fn drawn_expected_card(name: &str, card: u8) -> String {
    format!("{} has drawn a {}!! Keep asking!", name, card)
}

pub fn game_status(players: &[Player], cards: usize) -> String {
    format!(
        "GAME STATUS:\n\nPlayers info:\n\n{}\n\nDeck remaining cards: {}",
        players
            .iter()
            .map(|p| format!(
                "{} => Score: {}, Cards: {}",
                p.name.clone(),
                p.score,
                p.cards.len()
            ))
            .collect::<Vec<String>>()
            .join("\n\t"),
        cards
    )
}

pub fn ask_for_cards(from: &str, players: &[Player]) -> String {
    format!(
        "{} lets ask someone for a card😇:\n\nType '/ask <option> <card> with one of the following options:\n\n{}'",
        from,
        players.iter()
        .enumerate()        
        .map(|(index, player)| format!("{}) {}", index, player.name.clone()))
        .collect::<Vec<String>>()
        .join("\n")
    )
}

pub fn player_status(player: &Player) -> String {
    format!(
        "Hi {0}! Here is your status 😃:\n\tCards: {1}\n\tscore: {2}",
        player.name.clone(),
        player
            .cards
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(", "),
        player.score
    )
}

pub fn game_over(winners: &[String], score: u8) -> String {
    format!(
        "Game Over!\n\tWinners 👑: {}\n\tScore: {}",
        winners.join(", "),
        score
    )
}
