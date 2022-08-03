use std::path::PathBuf;

use clap::Subcommand;

#[derive(Subcommand, Debug)]
#[clap(about = "Deals with mqtt broker")]
pub enum MqttCommands {
    #[clap(help ="publish messages to mqtt topic", about ="publish messages to mqtt topic")]
    Pub {
        /// mqtt broker host
        #[clap(short='b', env = "MQTT_HOST", help="mqtt broker host")]
        host: String,

        /// mqtt broker port
        #[clap(short='c', env = "MQTT_PORT", help="mqtt broker port")]
        port: u16,

        /// mqtt topic
        #[clap(short = 't', env = "MQTT_TOPIC")]
        topic: String,

        /// publish config: yaml file path
        #[clap(short='m', parse(from_os_str))]
        publish_config: Option<PathBuf>
    },

    #[clap(help = "subscribe to mqtt topic ", about = "subscribe to mqtt topic ")]
    Sub {
        /// mqtt broker host
        #[clap(short='b', env = "MQTT_HOST")]
        host: String,

        /// mqtt broker port
        #[clap(short='c', env = "MQTT_PORT")]
        port: u16,

        /// mqtt topic
        #[clap(short='t', env = "MQTT_TOPIC")]
        topic: String,
    }, 
}


