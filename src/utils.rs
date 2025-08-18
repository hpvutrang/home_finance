use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

pub const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.fZ";

pub fn datefmt_deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    DateTime::parse_from_str(&s, FORMAT)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(serde::de::Error::custom)
}

// Date format for serialization
pub fn datefmt_serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date.format(FORMAT).to_string())
}