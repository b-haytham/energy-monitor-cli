use clap::{Parser, Subcommand};

mod mqtt;
mod db;

pub use mqtt::*;
pub use db::*;

#[derive(Parser, Debug)]
#[clap(
    author = "haytham boussarsar", 
    version, 
    about = "cli application to work with Energy Monitor app in development"
)]
pub struct Args {
    #[clap(subcommand)]
    pub command: ArgCommands,
}

#[derive(Subcommand, Debug)]
pub enum ArgCommands {

    #[clap(subcommand)]
    Mqtt(MqttCommands),

    #[clap(subcommand)]
    Db(DbCommands),
}


