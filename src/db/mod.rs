use std::str::FromStr;

use mongodb::Client;
use mongodb::options::ClientOptions;

pub struct Database {
    client: Client,
    database: String,
    collection: String
}

impl Database {
    pub async fn new(uri: &str, database: &str, collection: &str, repl_set: &str) -> anyhow::Result<Self> {
        let mut client_options = ClientOptions::parse(uri).await?;

        client_options.direct_connection = Some(true);
        client_options.repl_set_name = Some(String::from(repl_set));
        client_options.default_database = Some(String::from_str(database)?);

        Ok(Self{
            client: Client::with_options(client_options)?,
            database: String::from_str(database)?,
            collection: String::from_str(collection)?
        })
    }

    pub async fn list_collections(&self) -> anyhow::Result<Vec<String>> {
        let client = &self.client;
        let database = &self.database;
        let _collection = &self.collection;

        let names = client.database(database).list_collection_names(None).await?;
        Ok(names)
    }
}
