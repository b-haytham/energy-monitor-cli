use clap::Subcommand;
use clap;

#[derive(Subcommand, Debug)]
#[clap(about = "Deals with mqtt broker")]
pub enum MqttCommands {
    #[clap(help ="publish messages to mqtt broker", about ="publish messages to mqtt broker")]
    Pub {
        #[clap(short='b', env = "MQTT_HOST")]
        host: String,

        #[clap(short = 't', env = "MQTT_TOPIC")]
        topic: String,
    },

    #[clap(help = "subscribe mqtt to messages", about = "subscribe mqtt to messages")]
    Sub {
        #[clap(short='b', env = "MQTT_HOST")]
        host: String,

        #[clap(short='t', env = "MQTT_TOPIC")]
        topic: String,
    }, 
}


