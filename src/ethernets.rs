
use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::common;
use crate::common_physical;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ethernet {
    #[serde(deserialize_with = "common::deserialize_renderer", default)]
    renderer: Option<String>,
    #[serde(deserialize_with = "common::deserialize_boolean", default)]
    dhcp4: Option<String>,
    #[serde(deserialize_with = "common::deserialize_boolean", default)]
    dhcp6: Option<String>,
    #[serde(rename = "match", default)]
    matching: Option<common_physical::Match>,

    addresses: Option<Vec<common::IPAddress>>,
}


