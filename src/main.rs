
use std::{fs, collections::BTreeMap, fmt::Debug};

use serde_yaml;
use serde::{Serialize, Deserialize};

pub mod common;
pub mod common_physical;
pub mod ethernets;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Network {
    #[serde(deserialize_with = "common::deserialize_version")]
    version: Option<u8>,
    #[serde(deserialize_with = "common::deserialize_renderer")]
    renderer: Option<String>,

    ethernets: Option<BTreeMap<String, ethernets::Ethernet>>,
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
            println!("Pau em {:?}", err);
            return;
        }
    };

    println!("{a:#?}");
    
}
