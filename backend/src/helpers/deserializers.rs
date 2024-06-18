use serde::{Deserialize, Deserializer};

pub fn empty_string_is_invalid<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Err(serde::de::Error::custom("empty string is invalid"))
    } else {
        Ok(s)
    }
}
