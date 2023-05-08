use teloxide_core::{requests::Requester, types::Message, Bot};

use crate::types::{
    ecryption_state::FileEncryptionType,
    state::{HandlerResult, MyDialogue, State},
};

pub async fn receive_encryption_type(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            if text.to_lowercase() == "aes" {
                dialogue
                    .update(State::SetEncryptionType(FileEncryptionType::AES))
                    .await
                    .unwrap();
                bot.send_message(msg.chat.id, "Your type is AES.").await?;
                bot.send_message(msg.chat.id, "Now send the file").await?;
            } else if text.to_lowercase() == "none" {
                dialogue
                    .update(State::SetEncryptionType(FileEncryptionType::None))
                    .await
                    .unwrap();
                bot.send_message(msg.chat.id, "Your type is None.").await?;
                bot.send_message(msg.chat.id, "Now send the file").await?;
            } else {
                bot.send_message(msg.chat.id, "Invalid type").await?;
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Please, write type of encryption(AES or None)")
                .await?;
        }
    }

    Ok(())
}
