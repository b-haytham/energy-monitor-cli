use std::str::FromStr;

use bson::oid::ObjectId;
use serde::Serialize;

 
#[derive(Debug, Serialize, Clone)]
pub struct ValueSource {
    s: ObjectId,
    d: ObjectId,
    v: String,
}

impl ValueSource {
    // pub fn new(s: ObjectId, d: ObjectId, v: String) -> Self {
    //     Self { s, d, v }
    // }

    pub fn from_str(s: &str, d: &str, v: &str) -> anyhow::Result<Self> {
        Ok(Self {
            s: ObjectId::from_str(s)?,
            d: ObjectId::from_str(d)?,
            v: v.to_string(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Value {
    s: ValueSource,
    v: f64,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    t: chrono::DateTime<chrono::Utc>,
}

impl Value {
    pub fn new(s: ValueSource, v: f64, t: chrono::DateTime<chrono::Utc>) -> Self {
        Self { s, v, t }
    }
}

