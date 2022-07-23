use std::path::PathBuf;
use tokio::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeviceValueInfo {
    pub name: String,
    pub value: Option<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct DeviceInfo {
    pub id: String,
    pub token: String,
    pub repeat: bool,
    pub topic: String,
    pub interval: u64,
    pub values: Vec<DeviceValueInfo>
}

#[derive(Deserialize, Debug)]
pub struct PublishConfig {
    pub devices: Vec<DeviceInfo>
}

impl PublishConfig {
    pub async fn parse_from_path(path: Option<PathBuf>) -> anyhow::Result<Self> {
        let seed_config_path = match path {
            Some(path) => path,
            None => PathBuf::from("./configs/publish_config.yaml")
        };

        let file_content = fs::read_to_string(seed_config_path).await?;

        let seed_config: Self = serde_yaml::from_str(&file_content)?;
        Ok(seed_config)
    }
}
