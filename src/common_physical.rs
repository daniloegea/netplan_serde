use std::fmt::Debug;

use regex::Regex;

use serde::{Serialize, Deserialize, Deserializer, de };


fn deserialize_macaddress<'de, D>(d: D) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de> {

    let re = Regex::new(r"[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}").unwrap();

    let value = String::deserialize(d)?;

    if !re.is_match(&value) {
        return Err(de::Error::invalid_value(de::Unexpected::Str(&value), &"macaddress not well formed"));
    }

    Ok(Some(value))

}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Match {
    name: Option<String>,
    driver: Option<String>,
    #[serde(deserialize_with = "deserialize_macaddress")]
    macaddress: Option<String>,
}


