use teloxide::utils::command::BotCommand;

// Derive BotCommand to parse text with a command into this enumeration.
//
//  1. rename = "lowercase" turns all the commands into lowercase letters.
//  2. `description = "..."` specifies a text before all the commands.
//
// That is, you can just call Command::descriptions() to get a description of
// your commands in this format:
// %GENERAL-DESCRIPTION%
// %PREFIX%%COMMAND% - %DESCRIPTION%
#[derive(BotCommand, Debug, Clone)]
#[command(
    rename = "lowercase",
    description = "Use commands in format /command <arg1> <arg2> ... <argN> ",
    parse_with = "split"
)]
pub enum Command {
    #[command(description = "create a new game")]
    NewGame,
    #[command(description = "join the current game")]
    Join,
    #[command(description = "start the game")]
    Start,
    #[command(description = "end the game")]
    EndGame,
    #[command(description = "ask someone for cards")]
    Ask,
    #[command(description = "ask the bot to show the game general status")]
    Status,
    #[command(description = "ask the bot to send you your status")]
    MyStatus,
    #[command(description = "Show bot commands")]
    Help,
}
