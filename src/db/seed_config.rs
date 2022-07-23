use std::path::PathBuf;

use serde::Deserialize;
use tokio::fs;

#[derive(Deserialize, Debug)]
pub enum ValueSeedOptions {
    Add,
    Random
}

#[derive(Deserialize, Debug)]
pub struct DeviceInfo {
    pub subscription_id: String,
    
    pub device_id: String,

    pub device_name: Option<String>,

    pub value_name: String,

    pub value_create_option: ValueSeedOptions,

    pub months: u64,

    pub interval: u8,
}

#[derive(Deserialize, Debug)]
pub struct SeedConfig {
    pub devices: Vec<DeviceInfo>
}

impl SeedConfig {
    pub async fn parse_from_path(path: Option<PathBuf>) -> anyhow::Result<Self> {
        let seed_config_path = match path {
            Some(path) => path,
            None => PathBuf::from("./configs/seed_config.yaml")
        };

        let file_content = fs::read_to_string(seed_config_path).await?;

        let seed_config: Self = serde_yaml::from_str(&file_content)?;
        Ok(seed_config)
    }
}
