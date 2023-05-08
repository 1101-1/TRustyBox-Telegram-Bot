use teloxide::utils::command::BotCommands;
use teloxide_core::{requests::Requester, types::Message, Bot};

use crate::types::state::{HandlerResult, MyDialogue, State};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display commands.")]
    Help,
    #[command(description = "With whis command, you can get file by short_path")]
    GetFile,
    #[command(description = "Start uploading file.")]
    UploadFile,
    #[command(description = "Return to main menu")]
    Cancel,
}

pub async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    dialogue: MyDialogue,
) -> HandlerResult {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::GetFile => {
            dialogue.update(State::SendFileInfo).await?;
            bot.send_message(msg.chat.id, "Send <short_path> and <aes_key> if required")
                .await?
        }
        Command::UploadFile => {
            dialogue.update(State::HandleFile).await?;
            bot.send_message(
                msg.chat.id,
                "Choose and send your encryption type for upload file: Aes or None(default)",
            )
            .await?
        }
        Command::Cancel => {
            dialogue.exit().await?;
            bot.send_message(msg.chat.id, "Cancel command").await?
        }
    };
    Ok(())
}

pub async fn invalid_command(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Please, send /help to show available commands")
        .await?;
    Ok(())
}
