use std::env;

use dotenv::dotenv;
use teloxide::{dispatching::UpdateFilterExt, dptree};
use teloxide::{
    dispatching::{dialogue::InMemStorage, HandlerExt},
    prelude::Dispatcher,
};
use teloxide::{
    types::{Message, Update},
    Bot,
};

use crate::bot::command::{command_handler, invalid_command, Command};
use crate::bot::encrypt_type::receive_encryption_type;
use crate::bot::file_info::get_file_info;
use crate::bot::get_file::file_handler;
use crate::types::state::State;

mod bot;
mod crypt;
mod db;
mod tools;
mod types;

#[tokio::main]
async fn main() {
    dotenv().ok();

    pretty_env_logger::init();

    log::info!("Starting bot");

    let bot = Bot::new(env::var("BOT_TOKEN").unwrap());

    let handler = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
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
