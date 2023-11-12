use std::{collections::BTreeMap, fmt::Debug, net::IpAddr, str::FromStr};

use serde::{Serialize, Deserialize, Deserializer, de };

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum IpOptionLifetime {
    Interger(u64),
    Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IPOptions {
    lifetime: Option<IpOptionLifetime>,
    label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IPAddress {
    #[serde(deserialize_with = "deserialize_ipaddress")]
    Ip(String),
    #[serde(deserialize_with = "deserialize_ipaddress_with_options")]
    IpWithOptions(BTreeMap<String, IPOptions>),
}

fn validate_ipaddress(ipv4: &str) -> Result<(), ()> {

    let parts: Vec<&str> = ipv4.split('/').collect();
    if parts.len() != 2 {
        return Err(());
    }
    let ip = IpAddr::from_str(parts[0]);

    println!("{:?}", ip);
    match ip {
        Ok(_) => return Ok(()),
        Err(_) => return Err(()),
    };

}

fn deserialize_ipaddress<'de, D>(d: D) -> Result<String, D::Error>
    where D: Deserializer<'de> {

    let address: String = String::deserialize(d)?;
    if let Err(_) = validate_ipaddress(&address) {
        return Err(de::Error::invalid_value(de::Unexpected::Str(&address), &"must be an IP"));
    }

    Ok(address)

}

fn deserialize_ipaddress_with_options<'de, D>(d: D) -> Result<BTreeMap<String, IPOptions>, D::Error>
    where D: Deserializer<'de> {

    let address: BTreeMap<String, IPOptions> = BTreeMap::deserialize(d)?;

    let keys: Vec<&String> = address.keys().collect();

    if let Err(_) = validate_ipaddress(&keys[0]) {
        return Err(de::Error::invalid_value(de::Unexpected::Str(&keys[0]), &"must be an IP"));
    }

    Ok(address)
}

pub fn deserialize_boolean<'de, D>(d: D) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de> {

    let value = String::deserialize(d)?;

    if value != "true" && value != "false" && value != "yes" && value != "no" {
        return Err(de::Error::invalid_value(de::Unexpected::Str(&value), &"version must be 2"));
    }

    Ok(Some(value))

}

pub fn deserialize_version<'de, D>(d: D) -> Result<Option<u8>, D::Error>
    where D: Deserializer<'de> {

    let version = u8::deserialize(d)?;

    if version != 2 {
        return Err(de::Error::invalid_value(de::Unexpected::Unsigned(version as u64), &"version must be 2"));
    }

    Ok(Some(version))

}

pub fn deserialize_renderer<'de, D>(d: D) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de> {

    let renderer = String::deserialize(d)?;

    if renderer != "NetworkManager" && renderer != "networkd" {
        return Err(de::Error::invalid_value(de::Unexpected::Str(&renderer), &"renderer must be either NetworkManager or networkd"));
    }

    Ok(Some(renderer))

}


