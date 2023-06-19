use teloxide::{requests::Requester, types::Message, Bot};

use crate::{
    bot::file_actions::send_file::send_file,
    types::state::{HandlerResult, MyDialogue, State},
};

pub async fn get_file_info(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    if let Some(text) = msg.text() {
        let words: Vec<String> = text.split(" ").map(|str| str.to_string()).collect();
        if words.len() < 3 && words.len() > 0 {
            let short_path = words[0].to_string();
            let aes_key = words.get(1).or(None);
            if short_path.len() < 8 || short_path.len() > 8 {
                bot.send_message(msg.chat.id, "Short path invalid").await?;
                dialogue.update(State::SendFileInfo).await?;
                return Err("Short path invalid".into());
            }
            if let Some(aes_key) = aes_key {
                if aes_key.len() < 43 || aes_key.len() > 43 {
                    bot.send_message(msg.chat.id, "Aes key length is invalid")
                        .await?;
                    dialogue.update(State::SendFileInfo).await?;
                    return Err("Aes key length is invalid".into());
                }
            }
            send_file(bot, msg, short_path, aes_key.cloned(), dialogue.clone()).await?;
        } else {
            bot.send_message(msg.chat.id, "Invalid arguments").await?;
            dialogue.update(State::SendFileInfo).await?;
        }
    } else {
        bot.send_message(msg.chat.id, "Text is empty").await?;
        dialogue.update(State::SendFileInfo).await?;
    }
    Ok(())
}
