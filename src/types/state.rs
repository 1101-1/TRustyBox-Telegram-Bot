use teloxide::{dispatching::dialogue::InMemStorage, prelude::Dialogue};

use crate::types::ecryption_state::FileEncryptionType;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    HandleCommand,
    HandleFile,
    SetEncryptionType(FileEncryptionType),
    SendFileInfo,
}

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
