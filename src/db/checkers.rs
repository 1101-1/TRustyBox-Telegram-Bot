use async_recursion::async_recursion;
use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client, Collection,
};
use std::env;

use crate::tools::short_url::generate_short_path_url;

#[async_recursion]
pub async fn find_dublicate(short_url: String) -> String {
    let client_options = ClientOptions::parse(env::var("MONGO").expect("MONGO_ADDR doesn't set"))
        .await
        .unwrap();

    let client = Client::with_options(client_options).unwrap();

    let db = client.database(
        env::var("DATABASE_NAME")
            .expect("DATABASE_NAME doesn't set")
            .as_str(),
    );

    let collection: Collection<Document> = db.collection(
        env::var("COLLECTION_NAME")
            .expect("COLLECTION_NAME doesn't set")
            .as_str(),
    );

    if let Some(_url) = collection
        .find_one(doc! {"short_url": &short_url}, None)
        .await
        .unwrap()
    {
        find_dublicate(generate_short_path_url()).await;
    }

    short_url
}
