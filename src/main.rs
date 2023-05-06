use std::env;

use teloxide::{prelude::{Dispatcher, Dialogue}, dispatching::{dialogue::InMemStorage, HandlerExt}};
use dotenv::dotenv;
use teloxide::{dptree, dispatching::UpdateFilterExt};
use teloxide_core::{Bot, types::{Update, Message}, requests::{Requester}, net::Download};
use tokio::fs::File;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
enum State {
    #[default]
    Start,
    RecieveEncryptionType,
    SetEncryptionType(FileEncryptionType)
}

#[derive(Clone, PartialEq)]
enum FileEncryptionType {
    AES,
    None
}

async fn command_handler(
    msg: Message,
    bot: Bot,
    dialogue: MyDialogue
) -> HandlerResult {
    bot.send_message(msg.chat.id, "Choose your encryption type for upload file(AES or by default None)").await?;
    dialogue.update(State::RecieveEncryptionType).await?;
    Ok(())
}


async fn receive_encryption_type(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            if text.to_lowercase() == "aes" {
                dialogue.update(State::SetEncryptionType(FileEncryptionType::AES)).await.unwrap();
                bot.send_message(msg.chat.id, "Your type is AES.").await?;
                bot.send_message(msg.chat.id, "Now send the file").await?;
            } else {
                dialogue.update(State::SetEncryptionType(FileEncryptionType::None)).await.unwrap();
                bot.send_message(msg.chat.id, "Your type is None.").await?;
                bot.send_message(msg.chat.id, "Now send the file").await?;
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Please, write type of encryption(AES or None)").await?;
        }
    }

    Ok(())
}

async fn file_handler(
    msg: Message,
    bot: Bot,
    file_encryption_type: FileEncryptionType,
    dialogue: MyDialogue
) -> HandlerResult {
    if let Some(file) = msg.document() {
        if file_encryption_type ==  FileEncryptionType::AES {
            let file_id = &file.file.id;
            let telegram_file = bot.get_file(file_id.clone()).await?;
            let file_name = file.clone().file_name.unwrap_or(telegram_file.clone().meta.id);
            let path_for_file = format!("{}{}", env::var("PATH_TO_FILES").expect("VAR DOESN'T SET"), &file_name);
            log::info!("Downloading file {}", &path_for_file);
            let mut dst = File::create(&path_for_file).await?;
            bot.download_file(&telegram_file.path, &mut dst).await?;
            bot.send_message(msg.chat.id, "Your file succesfully download").await.unwrap();

            dialogue.exit().await?;
        } else {
            let file_id = &file.file.id;
            let telegram_file = bot.get_file(file_id.clone()).await?;
            let file_name = file.clone().file_name.unwrap_or(telegram_file.clone().meta.id);
            let path_for_file = format!("{}{}", env::var("PATH_TO_FILES").expect("VAR DOESN'T SET"), &file_name);
            log::info!("Downloading file {}", &path_for_file);
            let mut dst = File::create(&path_for_file).await?;
            bot.download_file(&telegram_file.path, &mut dst).await?;
            bot.send_message(msg.chat.id, "Your file succesfully download").await.unwrap();

            dialogue.exit().await?;
        }
    }
    if let Some(_pic) = msg.photo() {
        bot.send_message(msg.chat.id, "Send this file as telegram document. Not a \"photo or video\" option.").await.unwrap();
    }
    if let Some(_text) = msg.text() {
        bot.send_message(msg.chat.id, "To send file for upload, just send it").await.unwrap();
    }
    Ok(())
}


#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();

    log::info!("Starting bot");

    let bot = Bot::new(env::var("BOT_TOKEN").unwrap());
    Dispatcher::builder(
        bot, 
        Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(dptree::case![State::Start].endpoint(command_handler))
        .branch(dptree::case![State::RecieveEncryptionType].endpoint(receive_encryption_type))
        .branch(dptree::case![State::SetEncryptionType(file_encryption_type)].endpoint(file_handler))
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}