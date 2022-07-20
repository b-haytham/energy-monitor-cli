use clap::Parser;
use log::{trace, debug, info};
use dotenv;

use env_logger::Env;

mod args;
mod db;

use args::*;
use db::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::from_path(".env").ok();
    let env = Env::default()
        .filter_or("LOG_LEVEL", "trace")
        .write_style_or("LOG_STYLE", "always");

    env_logger::init_from_env(env);

    trace!("parsing args");
    let args = Args::parse();
    
    let command = args.command;

    match command {
        ArgCommands::Mqtt(mqtt_command) => {
            match mqtt_command {
                MqttCommands::Pub { host, topic } => {
                    debug!("mqtt publish command : host {host} | topic : {topic}");
                },
                MqttCommands::Sub { host, topic } => {
                    debug!("mqtt subscribe command : host {host} | topic : {topic}");
                }
            }
        },
        ArgCommands::Db(db_command) => {
            match db_command {
                DbCommands::Seed {
                    uri,
                    database,
                    collection,
                    repl_set
                } => {
                    debug!("db seed command : database {} | collection {}", &database, &collection);
                    let db = Database::new(&uri, &database, &collection, &repl_set).await?;
                    let names = db.list_collections().await?;
                    info!("collection names >> {:?}", names);
                },
                DbCommands::Drop {
                    uri,
                    database,
                    collection,
                    repl_set
                } => {
                    debug!("db drop command : database {database} | collection {collection}");
                    let db = Database::new(&uri, &database, &collection, &repl_set).await?;
                    info!("Connected to database");
                    let names = db.list_collections().await?;
                    info!("collection names >> {:?}", names);
                }
            }
        }
    }

    Ok(())
}
