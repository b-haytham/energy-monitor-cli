use std::path::PathBuf;
use std::str::FromStr;

use log::{info, debug};

use mongodb::Client;
use mongodb::options::ClientOptions;

use bson::doc;
use rand::Rng;

pub mod models;
mod seed_config;


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


    // test client ?
    pub async fn list_collections(&self) -> anyhow::Result<Vec<String>> {
        let client = &self.client;
        let database = &self.database;
        let _collection = &self.collection;

        let names = client.database(database).list_collection_names(None).await?;
        Ok(names)
    }

    // drop values for all devices
    pub async fn drop(&self) -> anyhow::Result<u64> {
        let client = &self.client;
        let database = &self.database;
        let collection = &self.collection;
        
        let result = client
            .database(database) 
            .collection::<models::Value>(collection)
            .delete_many(doc!{}, None) 
            .await?;

        Ok(result.deleted_count)
    }

    pub async fn parse_seed_config_from_path(&self, path: Option<PathBuf>) -> anyhow::Result<seed_config::SeedConfig> {
        let config = seed_config::SeedConfig::parse_from_path(path).await?;
        Ok(config)
    }

    pub async fn seed_device(&self, device_info: &seed_config::DeviceInfo) -> anyhow::Result<()> {
        let client = &self.client;
        let database = &self.database;
        let collection = &self.collection;
        

        let storage_coll = client.database(database).collection::<models::Value>(collection);
            
        
        let value_source = models::ValueSource::from_str(
            &device_info.subscription_id,
            &device_info.device_id,
            &device_info.value_name,
        )?;

        let mut past = chrono::Utc::now() - chrono::Duration::days(30 * *&device_info.months as i64);
        debug!("Past data {:?}", &past);
        let now = chrono::Utc::now();

        let mesg_interval = &device_info.interval;

        let mut range = rand::rngs::OsRng::default();
    
        let mut val = 0.0;

        while past < now {
            const NUM_VALUES_PER_INSERT: usize = 200000;

            let mut arr: Vec<models::Value> = Vec::with_capacity(NUM_VALUES_PER_INSERT);

            if past >= now {
                break;
            }

            for _ in 0..NUM_VALUES_PER_INSERT - 1 {
                let value = models::Value::new(value_source.clone(), val.clone(), past.clone());
                arr.push(value);


                // update variables
                
                val = match &device_info.value_create_option {
                    seed_config::ValueSeedOptions::Add => {
                        val + range.gen_range(0.0..120.0)
                    },
                    seed_config::ValueSeedOptions::Random => {
                        range.gen_range(0.0..120.0)
                    }
                };
                past = past + chrono::Duration::seconds(*mesg_interval as i64);

            }

            // insert
            let result = storage_coll.insert_many(&arr, None).await?;
            let dev_name = match &device_info.device_name {
                Some(name) => name,
                None => &device_info.device_id,
            };
            info!("Inserted {} documents for {}", result.inserted_ids.len(), dev_name);
            arr.clear();
        }

        Ok(())
    }
}
