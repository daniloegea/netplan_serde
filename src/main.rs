use std::{fs, collections::BTreeMap, fmt::Debug};

use serde_yaml;
use serde::{Serialize, Deserialize};

pub mod common;
pub mod common_physical;
pub mod ethernets;
pub mod modems;
pub mod wifis;
pub mod bridges;
pub mod dummy_devices;
pub mod bonds;
pub mod tunnels;
pub mod virtual_ethernets;
pub mod vlans;
pub mod vrfs;
pub mod nm_devices;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Network {
    #[serde(deserialize_with = "common::deserialize_version")]
    version: Option<u8>,
    #[serde(deserialize_with = "common::deserialize_renderer")]
    renderer: Option<String>,
    ethernets: Option<BTreeMap<String, ethernets::Ethernet>>,
    modems: Option<BTreeMap<String, modems::Modem>>,
    wifis: Option<BTreeMap<String, wifis::Wifi>>,
    bridges: Option<BTreeMap<String, bridges::Bridge>>,
    #[serde(rename = "dummy-devices")]
    dummy_devices: Option<BTreeMap<String, dummy_devices::DummyDevice>>,
    bonds: Option<BTreeMap<String, bonds::Bond>>,
    tunnels: Option<BTreeMap<String, tunnels::Tunnel>>,
    #[serde(rename = "virtual-ethernets")]
    virtual_ethernets: Option<BTreeMap<String, virtual_ethernets::VirtualEthernet>>,
    vlans: Option<BTreeMap<String, vlans::Vlan>>,
    vrfs: Option<BTreeMap<String, vrfs::Vrf>>,
    #[serde(rename = "nm-devices")]
    nm_devices: Option<BTreeMap<String, nm_devices::NmDevice>>,
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
