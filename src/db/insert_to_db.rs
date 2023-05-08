use crate::tools::short_url::generate_short_path_url;
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::env;

use crate::db::find_info::find_dublicate;

pub async fn insert_to_mongodb(
    path_download: &String,
    new_filename: &String,
    first_name: &str,
    mut short_path_url: String,
    is_aes: bool,
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
        short_path_url = find_dublicate(generate_short_path_url().await).await;
    };

    let document = doc! {
        "path": path_download,
        "first_name": first_name,
        "new_filename": new_filename,
        "short_url": short_path_url,
        "is_aes": is_aes
    };
    collection.insert_one(document, None).await.unwrap();

    Ok(())
}
