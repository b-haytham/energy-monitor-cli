use std::path::PathBuf;

use clap::Subcommand;
use clap;

#[derive(Subcommand, Debug)]
#[clap(about = "Deals with mqtt broker")]
pub enum MqttCommands {
    #[clap(help ="publish messages to mqtt broker", about ="publish messages to mqtt broker")]
    Pub {
        #[clap(short='b', env = "MQTT_HOST")]
        host: String,

        #[clap(short='c', env = "MQTT_PORT")]
        port: u16,

        #[clap(short = 't', env = "MQTT_TOPIC")]
        topic: String,

        #[clap(short = 'u', env = "MQTT_USERNAME")]
        username: Option<String>,

        #[clap(short = 'p', env = "MQTT_PASSWORD")]
        password: Option<String>,

        #[clap(short='m', parse(from_os_str))]
        publish_config: Option<PathBuf>
    },

    #[clap(help = "subscribe mqtt to messages", about = "subscribe mqtt to messages")]
    Sub {
        #[clap(short='b', env = "MQTT_HOST")]
        host: String,

        #[clap(short='c', env = "MQTT_PORT")]
        port: u16,

        #[clap(short='t', env = "MQTT_TOPIC")]
        topic: String,

        #[clap(short = 'u', env = "MQTT_USERNAME")]
        username: Option<String>,

        #[clap(short = 'p', env = "MQTT_PASSWORD")]
        password: Option<String>
    }, 
}


