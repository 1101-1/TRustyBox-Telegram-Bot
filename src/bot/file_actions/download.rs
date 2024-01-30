use std::env;

use teloxide::{net::Download, requests::Requester, types::Message, Bot};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
    crypt::{
        aes_key::set_aes_key, base64_convert::convert_aes_to_base64, encryption::encrypt_data,
    },
    db::insert_to_db::insert_main_info,
    tools::{generate_uuid::generate_uuid_v4, short_url::generate_short_path_url},
    types::{
        ecryption_state::FileEncryptionType,
        state::{HandlerResult, MyDialogue},
    },
};

pub async fn download_file(
    msg: Message,
    bot: Bot,
    file_name: String,
    id: &String,
    file_encryption_type: FileEncryptionType,
    dialogue: MyDialogue,
) -> HandlerResult {
    let chat_id_msg = msg.chat.id;
    if file_encryption_type == FileEncryptionType::AES {
        let telegram_file = bot.get_file(id.clone()).await?;
        let file_name = file_name;

        let new_filename = match file_name.split('.').last() {
            Some(extension) => format!("{}.{}", generate_uuid_v4(), extension),
            None => generate_uuid_v4(),
        };
        let generated_short_path = generate_short_path_url();
        let file_path = format!(
            "{}{}",
            env::var("PATH_TO_FILES").expect("VAR DOESN'T SET"),
            new_filename
        );

        let mut dst = File::create(&file_path).await?;
        bot.download_file(&telegram_file.path, &mut dst).await?;

        let aes_key = set_aes_key();
        let encoded_key = convert_aes_to_base64(aes_key);

        let mut open_file = File::open(&file_path).await?;
        let mut file_data = Vec::new();
        open_file.read_to_end(&mut file_data).await?;

        let encrypted_data = match encrypt_data(&file_data, aes_key) {
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
        match insert_main_info(
            &file_path,
            &new_filename,
            &file_name,
            generated_short_path.clone(),
            true,
            chat_id_msg,
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
You can download file from bot by /getfile command\n
/help",
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
        let file_id = id;
        let telegram_file = bot.get_file(file_id.clone()).await?;
        let file_name = file_name;
        let new_filename = match file_name.split('.').last() {
            Some(extension) => format!("{}.{}", generate_uuid_v4(), extension),
            None => generate_uuid_v4(),
        };
        let generated_short_path = generate_short_path_url();
        let file_path = format!(
            "{}{}",
            env::var("PATH_TO_FILES").expect("VAR DOESN'T SET"),
            new_filename
        );

        let mut dst = File::create(&file_path).await?;
        bot.download_file(&telegram_file.path, &mut dst).await?;

        match insert_main_info(
            &file_path,
            &new_filename,
            &file_name,
            generated_short_path.clone(),
            false,
            chat_id_msg,
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
You can download file from bot by /getfile command\n
/help",
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
    Ok(())
}
