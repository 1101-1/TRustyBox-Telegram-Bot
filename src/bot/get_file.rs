use std::env;

use teloxide_core::{net::Download, requests::Requester, types::Message, Bot};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
    crypt::{
        aes_key::set_aes_key, base64_convert::convert_aes_to_base64, encryption::encrypt_data,
    },
    db::insert_to_mongo::insert_to_mongodb,
    tools::{generate_uuid::generate_uuid_v4, short_url::generate_short_path_url},
    types::{
        ecryption_state::FileEncryptionType,
        state::{HandlerResult, MyDialogue},
    },
};

pub async fn file_handler(
    msg: Message,
    bot: Bot,
    file_encryption_type: FileEncryptionType,
    dialogue: MyDialogue,
) -> HandlerResult {
    if let Some(file) = msg.document() {
        if file_encryption_type == FileEncryptionType::AES {
            let file_id = &file.file.id;
            let telegram_file = bot.get_file(file_id.clone()).await?;
            let file_name = file
                .clone()
                .file_name
                .unwrap_or(telegram_file.clone().meta.id);

            let new_filename = match file_name.split('.').last() {
                Some(extension) => format!("{}.{}", generate_uuid_v4().await, extension),
                None => generate_uuid_v4().await,
            };
            let generated_short_path = generate_short_path_url().await;
            let file_path = format!(
                "{}{}",
                env::var("PATH_TO_FILES").expect("VAR DOESN'T SET"),
                new_filename
            );

            let mut dst = File::create(&file_path).await?;
            bot.download_file(&telegram_file.path, &mut dst).await?;

            let aes_key = set_aes_key().await;
            let encoded_key = convert_aes_to_base64(aes_key).await;

            let mut open_file = File::open(&file_path).await?;
            let mut file_data = Vec::new();
            open_file.read_to_end(&mut file_data).await?;

            let encrypted_data = match encrypt_data(&file_data, aes_key).await {
                Ok(encrypted_data) => encrypted_data,
                Err(_err) => {
                    bot.send_message(msg.chat.id, "Unable to crypt file. Try again")
                        .await?;
                    return Err("Cryption file Error".into());
                }
            };

            let mut dst = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&file_path)
                .await?;
            dst.write_all(&encrypted_data).await?;
            match insert_to_mongodb(
                &file_path,
                &new_filename,
                &file_name,
                generated_short_path.clone(),
                true,
            )
            .await
            {
                Ok(()) => (),
                Err(_err) => {
                    bot.send_message(msg.chat.id, "Err to add info into db")
                        .await?;
                    return Err("Err to add info into db".into());
                }
            };
            bot.send_message(msg.chat.id, "Your file succesfully download")
                .await
                .unwrap();
            bot.send_message(
                msg.chat.id,
                format!(
                    "Short path: {}\n
                    Encryption key: {}\n
                    You can download file from bot by /getfile command",
                    &generated_short_path, &encoded_key
                ),
            )
            .await
            .unwrap();
            bot.send_message(
                msg.chat.id,
                format!(
                    "Also you can download file on site http://{}/{}/{}",
                    env::var("SERVER_ADDR").expect("ADDR NOT FOUND"),
                    &generated_short_path,
                    &encoded_key
                ),
            )
            .await
            .unwrap();
            dialogue.exit().await?;
        } else {
            let file_id = &file.file.id;
            let telegram_file = bot.get_file(file_id.clone()).await?;
            let file_name = file
                .clone()
                .file_name
                .unwrap_or(telegram_file.clone().meta.id);
            let new_filename = match file_name.split('.').last() {
                Some(extension) => format!("{}.{}", generate_uuid_v4().await, extension),
                None => generate_uuid_v4().await,
            };
            let generated_short_path = generate_short_path_url().await;
            let file_path = format!(
                "{}{}",
                env::var("PATH_TO_FILES").expect("VAR DOESN'T SET"),
                new_filename
            );

            let mut dst = File::create(&file_path).await?;
            bot.download_file(&telegram_file.path, &mut dst).await?;

            match insert_to_mongodb(
                &file_path,
                &new_filename,
                &file_name,
                generated_short_path.clone(),
                false,
            )
            .await
            {
                Ok(()) => (),
                Err(_err) => {
                    bot.send_message(msg.chat.id, "Err to add info into db")
                        .await?;
                    return Err("Err to add info into db".into());
                }
            };
            bot.send_message(msg.chat.id, "Your file succesfully download")
                .await
                .unwrap();
            bot.send_message(
                msg.chat.id,
                format!(
                    "Short path: {} \n
                    You can download file from bot by /getfile command",
                    &generated_short_path
                ),
            )
            .await
            .unwrap();
            bot.send_message(
                msg.chat.id,
                format!(
                    "Also you can download file on site http://{}/{}",
                    env::var("SERVER_ADDR").expect("ADDR NOT FOUND"),
                    &generated_short_path,
                ),
            )
            .await
            .unwrap();
            dialogue.exit().await?;
        }
    }
    if let Some(_pic) = msg.photo() {
        bot.send_message(
            msg.chat.id,
            "Send this file as telegram document. Not a \"photo or video\" option.",
        )
        .await
        .unwrap();
    }
    if let Some(_text) = msg.text() {
        bot.send_message(msg.chat.id, "To send file for upload, just send it")
            .await
            .unwrap();
    }
    if let Some(_video) = msg.video() {
        bot.send_message(
            msg.chat.id,
            "Send this file as telegram document. Not a \"photo or video\" option.",
        )
        .await
        .unwrap();
    }
    if let Some(_sticker) = msg.sticker() {
        bot.send_message(
            msg.chat.id,
            "Send this file as telegram document. Not a \"sticker or webp dockument\".",
        )
        .await
        .unwrap();
    }
    Ok(())
}
