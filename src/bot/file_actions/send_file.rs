use teloxide::{
    requests::Requester,
    types::{InputFile, Message},
    Bot,
};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    crypt::{base64_convert::convert_base64_to_aes, decryption::decrypt_data},
    db::get_info::get_name_and_path_of_file,
    types::state::{HandlerResult, MyDialogue, State},
};

pub async fn send_file(
    bot: Bot,
    msg: Message,
    short_path: String,
    aes_key: Option<String>,
    dialogue: MyDialogue,
) -> HandlerResult {
    bot.send_message(msg.chat.id, "Sending file..").await?;
    let (path_to_file, file_name, is_encrypted) = match get_name_and_path_of_file(short_path).await
    {
        Ok((file_path, file_name, is_encrypted)) => (file_path, file_name, is_encrypted),
        Err(_err) => {
            bot.send_message(msg.chat.id, "Unable to take data from db. Try again")
                .await?;
            return Err("Short path doesn't found".into());
        }
    };
    if let Some(key) = aes_key {
        let key_bytes = match convert_base64_to_aes(key) {
            Ok(key) => key,
            Err(_err) => {
                bot.send_message(msg.chat.id, "Invalid key. Try again")
                    .await?;
                return Err("Cannot convert key from base64".into());
            }
        };
        let mut file = File::open(&path_to_file).await?;
        let mut file_data = Vec::new();
        file.read_to_end(&mut file_data).await?;

        let data = decrypt_data(file_data, key_bytes).unwrap();

        bot.send_document(
            msg.chat.id,
            InputFile::file_name(InputFile::memory(data), file_name.clone()),
        )
        .await?;
        dialogue.exit().await?;
        return Ok(());
    }

    if is_encrypted == false {
        bot.send_document(
            msg.chat.id,
            InputFile::file_name(InputFile::file(&path_to_file), file_name.clone()),
        )
        .await?;
        dialogue.exit().await?;
        return Ok(());
    }

    bot.send_message(msg.chat.id, "Aes key required").await?;
    dialogue.update(State::SendFileInfo).await?;
    Ok(())
}
