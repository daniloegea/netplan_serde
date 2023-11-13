use std::{collections::BTreeMap, fmt::Debug, net::IpAddr, str::FromStr};

use regex::Regex;
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DHCPOverrides {
    #[serde(rename = "use-dns", default, deserialize_with = "deserialize_boolean")]
    use_dns: Option<String>,
    #[serde(rename = "use-ntp", default, deserialize_with = "deserialize_boolean")]
    use_ntp: Option<String>,
    #[serde(rename = "send-hostname", default, deserialize_with = "deserialize_boolean")]
    send_hostname: Option<String>,
    #[serde(rename = "use-hostname", default, deserialize_with = "deserialize_boolean")]
    use_hostname: Option<String>,
    #[serde(rename = "use-mtu", default, deserialize_with = "deserialize_boolean")]
    use_mtu: Option<String>,
    hostname: Option<String>,
    #[serde(rename = "use-routes", default, deserialize_with = "deserialize_boolean")]
    use_routes: Option<String>,
    #[serde(rename = "route-metric", default)]
    route_metric: Option<u64>,
    #[serde(rename = "use-domains", default)]
    use_domains: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Nameservers {
    search: Option<Vec<String>>,
    addresses: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Route {
    from: Option<String>,
    to: Option<String>,
    via: Option<String>,
    #[serde(rename = "on-link", default, deserialize_with = "deserialize_boolean")]
    on_link: Option<String>,
    metric: Option<u64>,
    #[serde(rename = "type", default)]
    route_type: Option<String>,
    scope: Option<String>,
    table: Option<u64>,
    mtu: Option<u64>,
    #[serde(rename = "congestion-window", default)]
    congestion_window: Option<u64>,
    #[serde(rename = "advertised-receive-window", default)]
    advertised_receive_window: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoutingPolicy {
    from: Option<String>,
    to: Option<String>,
    table: Option<u64>,
    priority: Option<u64>,
    mark: Option<u64>,
    #[serde(rename = "type-of-service", default)]
    type_of_service: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkManager {
    name: Option<String>,
    uuid: Option<String>,
    #[serde(rename = "stable-id", default)]
    stable_id: Option<String>,
    device: Option<String>,
    passthrough: Option<BTreeMap<String, String>>,
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

pub fn deserialize_link_local<'de, D>(d: D) -> Result<Option<Vec<String>>, D::Error>
    where D: Deserializer<'de> {

    let mut link_local: Vec<String> = Vec::deserialize(d)?;

    for val in &link_local {
        if val != "ipv4" && val != "ipv6" {
            return Err(de::Error::invalid_value(de::Unexpected::Str(&val), &"value must be either ipv4 or ipv6"));
        }
    }

    link_local.dedup();

    Ok(Some(link_local))

}

pub fn deserialize_dhcp_identifier<'de, D>(d: D) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de> {

    let identifier = String::deserialize(d)?;

    if identifier != "duid" && identifier != "mac" {
        return Err(de::Error::invalid_value(de::Unexpected::Str(&identifier), &"renderer must be either duid or mac"));
    }

    Ok(Some(identifier))

}

pub fn deserialize_ipv6_address_generation<'de, D>(d: D) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de> {

    let identifier = String::deserialize(d)?;

    if identifier != "euid64" && identifier != "stable-privacy" {
        return Err(de::Error::invalid_value(de::Unexpected::Str(&identifier), &"renderer must be either duid or mac"));
    }

    Ok(Some(identifier))

}

pub fn deserialize_macaddress<'de, D>(d: D) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de> {

    let re = Regex::new(r"[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}:[0-9a-fA-F]{2}").unwrap();

    let value = String::deserialize(d)?;

    if !re.is_match(&value) {
        return Err(de::Error::invalid_value(de::Unexpected::Str(&value), &"macaddress not well formed"));
    }

    Ok(Some(value))

}

pub fn deserialize_optional_addresses<'de, D>(d: D) -> Result<Option<Vec<String>>, D::Error>
    where D: Deserializer<'de> {

    let mut optional: Vec<String> = Vec::deserialize(d)?;

    for val in &optional {
        if val != "ipv4" && val != "ipv6" {
            return Err(de::Error::invalid_value(de::Unexpected::Str(&val), &"value must be either ipv4 or ipv6"));
        }
    }

    optional.dedup();

    Ok(Some(optional))

}

pub fn deserialize_activation_mode<'de, D>(d: D) -> Result<Option<String>, D::Error>
    where D: Deserializer<'de> {

    let value = String::deserialize(d)?;

    Ok(Some(value))
}
