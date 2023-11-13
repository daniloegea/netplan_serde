use std::fmt::Debug;

use std::collections::BTreeMap;

use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

use crate::common;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Match {
    name: Option<String>,
    driver: Option<String>,
    #[serde(default, deserialize_with = "common::deserialize_macaddress")]
    macaddress: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenvSwitch {
    #[serde(rename = "external-ids", default)]
    external_ids: Option<BTreeMap<String, String>>,
    #[serde(rename = "other-config", default)]
    other_config: Option<BTreeMap<String, String>>,
    lacp: Option<String>,
    #[serde(rename = "fail-mode", default)]
    fail_mode: Option<String>,
    #[serde(default, rename = "mcast-snooping", deserialize_with = "common::deserialize_boolean")]
    mcast_snooping: Option<String>,
    protocols: Option<Vec<String>>,
    #[serde(default, deserialize_with = "common::deserialize_boolean")]
    rstp: Option<String>,
    controller: Option<OpenvSwitchController>,
    ports: Option<Vec<Vec<String>>>,
    ssl: Option<OpenvSwitchSSL>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenvSwitchController {
    addresses: Option<Vec<String>>,
    #[serde(rename = "connection-mode")]
    connection_mode: Option<String>,
    ssl: Option<OpenvSwitchSSL>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenvSwitchSSL {
    #[serde(rename = "ca-cert")]
    ca_cert: Option<String>,
    certificate: Option<String>,
    #[serde(rename = "private-key")]
    private_key: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Auth {
    #[serde(rename = "key-management")]
    key_management: Option<String>,
    password: Option<String>,
    method: Option<String>,
    identity: Option<String>,
    #[serde(rename = "anonymous-identity")]
    anonymous_identity: Option<String>,
    #[serde(rename = "ca-certificate")]
    ca_certificate: Option<String>,
    #[serde(rename = "client-certificate")]
    client_certificate: Option<String>,
    #[serde(rename = "client-key")]
    client_key: Option<String>,
    #[serde(rename = "client-key-password")]
    client_key_password: Option<String>,
    #[serde(rename = "phase2-auth")]
    phase2_auth: Option<String>,
}
