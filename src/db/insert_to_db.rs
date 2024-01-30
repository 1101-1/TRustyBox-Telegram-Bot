use crate::tools::short_url::generate_short_path_url;
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;
use teloxide::types::ChatId;

use crate::db::checkers::find_dublicate;

pub async fn insert_main_info(
    path_download: &String,
    new_filename: &String,
    first_name: &str,
    mut short_path_url: String,
    is_aes: bool,
    id: ChatId,
) -> mongodb::error::Result<()> {
    let client_options = ClientOptions::parse(env::var("MONGO").expect("MONGO_ADDR doesn't set"))
        .await
        .unwrap();

    let client = Client::with_options(client_options).unwrap();

    let db = client.database(
        env::var("DATABASE_NAME")
            .expect("DATABASE_NAME doesn't set")
            .as_str(),
    );

    let collection = db.collection(
        env::var("COLLECTION_NAME")
            .expect("COLLECTION_NAME doesn't set")
            .as_str(),
    );

    if let Some(_url) = collection
        .find_one(doc! {"short_url": &short_path_url}, None)
        .await
        .unwrap()
    {
        short_path_url = find_dublicate(generate_short_path_url()).await;
    };

    let document = doc! {
        "path": path_download,
        "first_name": first_name,
        "new_filename": new_filename,
        "short_url": short_path_url.clone(),
        "is_aes": is_aes
    };
    collection.insert_one(document, None).await.unwrap();

    match insert_telegram_info(short_path_url, id).await {
        Ok(()) => return Ok(()),
        Err(err) => return Err(err.into()),
    };
}

async fn insert_telegram_info(short_path_url: String, id: ChatId) -> mongodb::error::Result<()> {
    let client_options = ClientOptions::parse(env::var("MONGO").expect("MONGO_ADDR doesn't set"))
        .await
        .unwrap();

    let client = Client::with_options(client_options).unwrap();

    let db = client.database(
        env::var("DATABASE_NAME")
            .expect("DATABASE_NAME doesn't set")
            .as_str(),
    );

    let collection = db.collection(
        env::var("COLLECTION_NAME_TELEGRAM")
            .expect("COLLECTION_NAME_TELEGRAM doesn't set")
            .as_str(),
    );

    let id_to_str = id.0;
    let document = doc! {
        "short_url": short_path_url,
        "telegram_id": id_to_str
    };
    collection.insert_one(document, None).await.unwrap();

    Ok(())
}
