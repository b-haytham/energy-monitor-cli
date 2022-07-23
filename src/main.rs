use clap::Parser;
use log::{trace, info};
use dotenv;

use env_logger::Env;

mod args;
mod db;
mod mqtt;

use args::*;
use db::*;
use mqtt::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::from_path(".env").ok();
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "always");

    env_logger::init_from_env(env);

    trace!("parsing args");
    let args = Args::parse();
    
    let command = args.command;

    match command {
        ArgCommands::Mqtt(mqtt_command) => {
            match mqtt_command {
                MqttCommands::Pub { 
                    host, 
                    port, 
                    topic, 
                    publish_config 
                } => {
                    info!("mqtt publish command : host {host} | topic : {topic}");
                    let mut mqtt_client = Mqtt::new(
                        &host, 
                        port, 
                    );
                    mqtt_client.publish(publish_config).await?;
                },
                MqttCommands::Sub { host, port, topic } => {
                    let topics = topic.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
                    info!("mqtt subscribe command : host {host} | topics : {:?}", &topics);
                    let mut mqtt_client = Mqtt::new(
                        &host, 
                        port, 
                    );

                    mqtt_client.subscribe(topics).await?
                }
            }
        },
        ArgCommands::Db(db_command) => {
            match db_command {
                DbCommands::Seed {
                    uri,
                    database,
                    collection,
                    repl_set,
                    seed_config,
                } => {
                    info!("db seed command : database {} | collection {}", &database, &collection);
                    let db = Database::new(&uri, &database, &collection, &repl_set).await?;
                    info!("Connected to database");
                    let names = db.list_collections().await?;
                    info!("collection names >> {:?}", names);
                    db.seed(seed_config).await?;
                    info!("Done seeding");
                },
                DbCommands::Drop {
                    uri,
                    database,
                    collection,
                    repl_set
                } => {
                    info!("db drop command : database {database} | collection {collection}");
                    let db = Database::new(&uri, &database, &collection, &repl_set).await?;
                    info!("Connected to database");
                    let names = db.list_collections().await?;
                    info!("collection names >> {:?}", names);
                    let deleted_count = db.drop().await?;
                    info!("Deleted {deleted_count} documents");
                }
            }
        }
    }

    Ok(())
}
