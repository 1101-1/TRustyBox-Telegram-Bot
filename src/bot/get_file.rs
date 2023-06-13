use teloxide::{requests::Requester, types::Message, Bot};

use crate::types::{
    ecryption_state::FileEncryptionType,
    state::{HandlerResult, MyDialogue},
};

use super::download::download_file;

pub async fn file_handler(
    msg: Message,
    bot: Bot,
    file_encryption_type: FileEncryptionType,
    dialogue: MyDialogue,
) -> HandlerResult {
    if let Some(file) = msg.document() {
        let file_id = &file.file.id;
        let file_name = file.clone().file_name.unwrap_or(file.clone().file.id);
        download_file(
            msg.clone(),
            bot.clone(),
            file_name,
            file_id,
            file_encryption_type.clone(),
            dialogue.clone(),
        )
        .await?;
    }
    if let Some(_pic) = msg.photo() {
        bot.send_message(
            msg.chat.id,
            "Send this file as \"telegram document\". Not a \"photo or video\" option.",
        )
        .await
        .unwrap();
    }
    if let Some(_text) = msg.text() {
        bot.send_message(msg.chat.id, "To send file for upload, just send it")
            .await
            .unwrap();
    }
    if let Some(video) = msg.video() {
        let file_id = &video.file.id;
        let file_name = video.clone().file_name.unwrap_or(String::from("video"));
        download_file(
            msg.clone(),
            bot.clone(),
            file_name,
            file_id,
            file_encryption_type.clone(),
            dialogue.clone(),
        )
        .await?;
    }
    if let Some(_sticker) = msg.sticker() {
        bot.send_message(
            msg.chat.id,
            "Send this file as telegram document. Not a sticker or webp.",
        )
        .await
        .unwrap();
    }
    Ok(())
}
