use std::env;

use crypt::base64_convert::convert_base64_to_aes;
use crypt::decryption::decrypt_data;
use crypt::{
    aes_key::set_aes_key, base64_convert::convert_aes_to_base64, encryption::encrypt_data,
};
use db::get_name_and_path_of_file::get_name_and_path_of_file;
use db::insert_to_mongo::insert_to_mongodb;
use dotenv::dotenv;
use teloxide::{dispatching::UpdateFilterExt, dptree};
use teloxide::{
    dispatching::{dialogue::InMemStorage, HandlerExt},
    prelude::{Dialogue, Dispatcher},
    utils::command::BotCommands,
};
use teloxide_core::types::InputFile;
use teloxide_core::{
    net::Download,
    requests::Requester,
    types::{Message, Update},
    Bot,
};
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::tools::{generate_uuid::generate_uuid_v4, short_url::generate_short_path_url};

mod crypt;
mod db;
mod tools;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
enum State {
    #[default]
    Start,
    HandleCommand,
    HandleFile,
    SetEncryptionType(FileEncryptionType),
    SendFileInfo,
}

#[derive(Clone, PartialEq)]
enum FileEncryptionType {
    AES,
    None,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display commands.")]
    Help,
    #[command(description = "With whis command, you can get file by short_path")]
    GetFile,
    #[command(description = "Start uploading file.")]
    UploadFile,
    #[command(description = "Return to main menu")]
    Cancel,
}

async fn command_handler(
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
            dialogue.update(State::HandleCommand).await?;
            bot.send_message(msg.chat.id, "Cancel command").await?
        }
    };

    Ok(())
}

async fn invalid_command(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Please, send /help to show available commands")
        .await?;
    Ok(())
}

async fn start(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "Write /help to show commands")
        .await?;
    dialogue.update(State::HandleCommand).await?;
    Ok(())
}

async fn get_file(
    bot: Bot,
    msg: Message,
    short_path: String,
    aes_key: Option<String>,
    dialogue: MyDialogue,
) -> HandlerResult {
    bot.send_message(msg.chat.id, "Sending file..").await?;
    let (path_to_file, file_name, is_encrypted) = match get_name_and_path_of_file(short_path).await {
        Ok((file_path, file_name, is_encrypted)) => (file_path, file_name, is_encrypted),
        Err(_err) => {
            bot.send_message(msg.chat.id, "Unable to take data from db. Try again")
                .await?;
            return Err("Short path doesn't found".into());
        }
    };
    if let Some(key) = aes_key {
        let key_bytes = match convert_base64_to_aes(key).await {
            Ok(key) => key,
            Err(_err) => {
                bot.send_message(msg.chat.id, "Cannot convert key from base64. Try again")
                    .await?;
                return Err("Cannot convert key from base64".into());
            }
        };
        let mut file = File::open(&path_to_file).await?;
        let mut file_data = Vec::new();
        file.read_to_end(&mut file_data).await?;

        let mut data = decrypt_data(&file_data, key_bytes).await.unwrap();

        let mut dst = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path_to_file)
            .await?;

        dst.write_all(&mut data).await?;

        bot.send_document(
            msg.chat.id,
            InputFile::file_name(InputFile::file(&path_to_file), file_name.clone()),
        )
        .await?;
        dialogue.update(State::HandleCommand).await?;
        return Ok(());
    }

    if is_encrypted == false {
        bot.send_document(
        msg.chat.id,
        InputFile::file_name(InputFile::file(&path_to_file), file_name.clone()),
    )
    .await?;
    dialogue.update(State::HandleCommand).await?;
    return Ok(());
    }

    bot.send_message(msg.chat.id, "Aes key is empty").await?;
    dialogue.update(State::SendFileInfo).await?;
    Ok(())
    
}

async fn get_file_info(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    if let Some(text) = msg.text() {
        let words: Vec<String> = text.split(" ").map(|str| str.to_string()).collect();
        if words.len() < 3 && words.len() > 0 {
            let short_path = words[0].to_string();
            let aes_key = words.get(1).or(None);
            if short_path.len() < 7 {
                bot.send_message(msg.chat.id, "Short path invalid").await?;
                dialogue.update(State::SendFileInfo).await?;
                return Err("Short path invalid".into());
            }
            if let Some(aes_key) = aes_key {
                if aes_key.len() < 32 && aes_key.len() > 32 {
                    bot.send_message(msg.chat.id, "Aes key length is invalid")
                        .await?;
                    dialogue.update(State::SendFileInfo).await?;
                    return Err("Aes key length is invalid".into());
                }
            }
            get_file(bot, msg, short_path, aes_key.cloned(), dialogue.clone()).await?;
            dialogue.update(State::HandleCommand).await?;
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

async fn receive_encryption_type(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
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

async fn file_handler(
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
                    "Short path: {} \n
                    Encryption key: {} \n
                    You can download file from bot by command /getfile command",
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

            dialogue.update(State::HandleCommand).await?;
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

            dialogue.update(State::HandleCommand).await?;
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
        bot.send_message(msg.chat.id, "Send this file as telegram document. Not a \"photo or video\" option.")
            .await
            .unwrap();
    }
    if let Some(_sticker) = msg.sticker() {
        bot.send_message(msg.chat.id, "Send this file as telegram document. Not a \"sticker or webp dockument\".")
            .await
            .unwrap();
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();

    log::info!("Starting bot");

    let bot = Bot::new(env::var("BOT_TOKEN").unwrap());

    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(dptree::case![State::Start].endpoint(start))
        .branch(
            dptree::case![State::HandleCommand]
                .branch(
                    dptree::entry()
                        .filter_command::<Command>()
                        .endpoint(command_handler),
                )
                .branch(dptree::endpoint(invalid_command)),
        )
        .branch(
            dptree::case![State::HandleFile]
                .branch(
                    dptree::entry()
                        .filter_command::<Command>()
                        .endpoint(command_handler),
                )
                .branch(dptree::endpoint(receive_encryption_type)),
        )
        .branch(
            dptree::case![State::SetEncryptionType(file_encryption_type)]
                .branch(
                    dptree::entry()
                        .filter_command::<Command>()
                        .endpoint(command_handler),
                )
                .branch(dptree::endpoint(file_handler)),
        )
        .branch(
            dptree::case![State::SendFileInfo]
                .branch(
                    dptree::entry()
                        .filter_command::<Command>()
                        .endpoint(command_handler),
                )
                .branch(dptree::endpoint(get_file_info)),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
