use std::{time::Duration, path::PathBuf, collections::HashMap, str::FromStr};

use futures::{future::try_join_all, StreamExt};
use log::{trace, info};
use rand::Rng;

mod models;
mod publish_config;

pub struct Mqtt {
    client: paho_mqtt::AsyncClient,
}

impl Mqtt {
     pub async fn new(host: &str, port: u16, username: Option<&str>, password: Option<&str>) -> anyhow::Result<Self> {
         
        let client = paho_mqtt::AsyncClient::new(format!("tcp://{}:{}", host, port))?;

        let mut connect_opt_builder = paho_mqtt::connect_options::ConnectOptionsBuilder::new();

        connect_opt_builder.keep_alive_interval(Duration::from_secs(20));
        
        match (username, password) {
            (Some(username), Some(password)) => {
                connect_opt_builder.user_name(username).password(password);
            },
            _ => {} 
        }
        
        let connect_opts = connect_opt_builder.finalize();
        client.connect(connect_opts).await?;
        Ok(Self { client })
    }

    pub async fn subscribe(&mut self, topics: Vec<String>) -> anyhow::Result<()> {
        let mut stream = self.client.get_stream(55);

        self.client.subscribe_many(&topics, &[1, topics.len() as i32]);
    
        while let Some(msg_opt) = stream.next().await {
            if let Some(msg) = msg_opt {
                info!("Recived {msg}");
            }
        }
        Ok(())
    }

    pub async fn publish(&mut self, publish_config_path: Option<PathBuf>) -> anyhow::Result<()> {
        let config = publish_config::PublishConfig::parse_from_path(publish_config_path).await?;
        trace!("parsed config >> {:?}", config);

        let mut futures = Vec::new();
        let device_infos = config.devices;

        for device_info in device_infos {
            let future = self.publish_device(device_info);
            futures.push(future)
        }

        try_join_all(futures).await?;

        Ok(())
    }

    async fn publish_device(&self, device_info: publish_config::DeviceInfo) -> anyhow::Result<()> {


        let message_payload = self.get_message_payload(&device_info.values)?;

        let mqtt_message = models::MqttMessage::new(
            &device_info.id, 
            message_payload, 
            &chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs , true)
        );

    
        if !device_info.repeat {
            self.client.publish(
                paho_mqtt::Message::new(&device_info.topic, mqtt_message.try_to_string()?, paho_mqtt::QOS_1)
            ).await?;
            info!("sent to topic {} payload {}", &device_info.topic, &mqtt_message.try_to_string()?);
            return Ok(());
        }

        loop {
            let message_payload = self.get_message_payload(&device_info.values)?;   
            let mqtt_message = models::MqttMessage::new(
                &device_info.id, 
                message_payload, 
                &chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs , true)
            );
            
            let res =  self.client.publish(
                paho_mqtt::Message::new(&device_info.topic, mqtt_message.try_to_string()?, paho_mqtt::QOS_1)
            ).await;
            
            match res {
                Ok(()) => {
                    info!("sent to topic {} payload {}", &device_info.topic, &mqtt_message.try_to_string()?);
                },
                Err(e) => {
                    info!("sent to topic {} payload {} failed | reason {}", &device_info.topic, &mqtt_message.try_to_string()?, e);
                },
            }

            tokio::time::sleep(Duration::from_secs(device_info.interval)).await;
        }
        
    }

    fn get_message_payload(&self, values_infos: &[publish_config::DeviceValueInfo]) -> anyhow::Result<HashMap<String, f64>> {
        let mut range = rand::rngs::OsRng::default();
        let mut message_payload: HashMap<String, f64> = HashMap::new();
        // populate payload
        for value_info in values_infos {
            match (value_info.min, value_info.max, value_info.value) {
                (.., Some(val)) => {
                    message_payload.insert(String::from_str(value_info.name.as_str())?, val);
                },
                (Some(min), Some(max), None) => {
                    trace!("value with min ({}) and max ({})", min, max);
                    message_payload.insert(String::from_str(value_info.name.as_str())?, range.gen_range(min..max));
                },
                _ => {
                    return Err(anyhow::anyhow!("Invalid value config {:?}", value_info));
                }

            }
        }

        Ok(message_payload)
    }
}
