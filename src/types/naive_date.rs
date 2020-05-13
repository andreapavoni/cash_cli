use chrono::NaiveDate;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S>(val: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    val.format("%Y-%m-%d").to_string().serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let val: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&val, "%Y-%m-%d").map_err(D::Error::custom)
}
