use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::common::deserialize_macaddress;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Match {
    name: Option<String>,
    driver: Option<String>,
    #[serde(deserialize_with = "deserialize_macaddress")]
    macaddress: Option<String>,
}


