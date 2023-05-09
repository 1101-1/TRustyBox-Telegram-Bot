use teloxide::{requests::Requester, types::Message, Bot};

use crate::{
    db::get_info::telegram_get_path,
    types::state::{HandlerResult, MyDialogue},
};

pub async fn send_history(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    let chat_id = msg.chat.id.0;
    match telegram_get_path(chat_id).await {
        Ok(short_path) => {
            bot.send_message(
                msg.chat.id,
                format!("Here is your result: [{}]", short_path.join("] [")),
            )
            .await?;
            dialogue.exit().await?;
            return Ok(());
        }
        Err(err) => {
            bot.send_message(msg.chat.id, "Seems like you doesn't sent any file")
                .await?;
            dialogue.exit().await?;
            return Err(err.into());
        }
    };
}
