
use std::{fs, collections::BTreeMap, fmt::Debug, net::IpAddr, str::FromStr};

use regex::Regex;

use serde_yaml;
use serde::{Serialize, Deserialize, Deserializer, de };

fn validate_macaddress<'de, D>(d: D) -> Result<Option<String>, D::Error>
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
struct Match {
    name: Option<String>,
    driver: Option<String>,
    #[serde(deserialize_with = "validate_macaddress")]
    macaddress: Option<String>,
}

fn validate_boolean<'de, D>(d: D) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de> {

    let value = String::deserialize(d)?;

    if value != "true" && value != "false" && value != "yes" && value != "no" {
        return Err(de::Error::invalid_value(de::Unexpected::Str(&value), &"version must be 2"));
    }

    Ok(Some(value))

}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum IpOptionLifetime {
    Interger(u64),
    Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct IPOptions {
    lifetime: Option<IpOptionLifetime>,
    label: Option<String>,
}

fn validate_ipv4_and_ipv6(ipv4: &str) -> Result<(), ()> {

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

fn validate_ipaddress<'de, D>(d: D) -> Result<String, D::Error>
    where D: Deserializer<'de> {

    let address: String = String::deserialize(d)?;
    if let Err(_) = validate_ipv4_and_ipv6(&address) {
        return Err(de::Error::invalid_value(de::Unexpected::Str(&address), &"must be an IP"));
    }

    Ok(address)

}

fn validate_ipaddress_with_options<'de, D>(d: D) -> Result<BTreeMap<String, IPOptions>, D::Error>
    where D: Deserializer<'de> {

    let address: BTreeMap<String, IPOptions> = BTreeMap::deserialize(d)?;

    let keys: Vec<&String> = address.keys().collect();

    if let Err(_) = validate_ipv4_and_ipv6(&keys[0]) {
        return Err(de::Error::invalid_value(de::Unexpected::Str(&keys[0]), &"must be an IP"));
    }

    Ok(address)
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum IPAddress {
    #[serde(deserialize_with = "validate_ipaddress")]
    Ip(String),
    #[serde(deserialize_with = "validate_ipaddress_with_options")]
    IpWithOptions(BTreeMap<String, IPOptions>),
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Ethernet {
    #[serde(deserialize_with = "validate_boolean")]
    dhcp4: Option<String>,
    #[serde(deserialize_with = "validate_boolean")]
    dhcp6: Option<String>,
    #[serde(rename = "match")]
    matching: Option<Match>,

    addresses: Option<Vec<IPAddress>>,
}

fn validate_version<'de, D>(d: D) -> Result<Option<u8>, D::Error>
    where D: Deserializer<'de> {

    let version = u8::deserialize(d)?;

    if version != 2 {
        return Err(de::Error::invalid_value(de::Unexpected::Unsigned(version as u64), &"version must be 2"));
    }

    Ok(Some(version))

}

fn validate_renderer<'de, D>(d: D) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de> {

    let renderer = String::deserialize(d)?;

    if renderer != "NetworkManager" && renderer != "networkd" {
        return Err(de::Error::invalid_value(de::Unexpected::Str(&renderer), &"renderer must be either NetworkManager or networkd"));
    }

    Ok(Some(renderer))

}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Network {
    #[serde(deserialize_with = "validate_version")]
    version: Option<u8>,
    #[serde(deserialize_with = "validate_renderer")]
    renderer: Option<String>,

    ethernets: Option<BTreeMap<String, Ethernet>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Netplan {
    network: Network,
}

fn main() {

    let yaml = fs::read_to_string("test.yaml").unwrap();

    let a: Netplan = match serde_yaml::from_str(&yaml) {
        Ok(n) => n,
        Err(err) => {
            println!("Pau em {:?}", err.location());
            return;
        }
    };

    println!("{a:#?}");
    
}
