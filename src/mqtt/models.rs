use std::collections::HashMap;

use serde::Serialize;


#[derive(Serialize, Debug)]
pub struct MqttMessage {
    //device id
    d: String,

    // payload
    p: HashMap<String, f64>,

    // timestamp
    t: String,
}

impl MqttMessage {
    pub fn new(d: &str, p: HashMap<String, f64>, t: &str) -> Self {
        Self { d: d.into(), p, t: t.into()}
    }

    pub fn try_to_string(&self) -> anyhow::Result<String> {
        let str = serde_json::to_string(self)?;
        Ok(str)
    }

}
