use mongodb::{
    bson::{doc, Document},
    options::ClientOptions,
    Client, Collection,
};
use std::env;

pub async fn get_name_and_path_of_file(
    db_short_url: String,
) -> mongodb::error::Result<(String, String, bool)> {
    let client_options =
        ClientOptions::parse(env::var("MONGO").expect("MONGO_ADDR doesn't set")).await?;

    let client = Client::with_options(client_options)?;

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

    if let Some(doc) = collection
        .find_one(doc! {"short_url": db_short_url}, None)
        .await?
    {
        let path = doc.get_str("path").unwrap().to_string();
        let first_name = doc.get_str("first_name").unwrap().to_string();
        let is_aes = doc.get_bool("is_aes").unwrap();
        return Ok((path, first_name, is_aes));
    } else {
        return Err(mongodb::error::Error::from(tokio::io::Error::new(
            tokio::io::ErrorKind::Other,
            "FILE not found or URL doesn't exist",
        )));
    }
}
