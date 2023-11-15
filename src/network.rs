use std::{collections::BTreeMap, fmt::Debug};

use crate::common_physical::OpenvSwitch;
use crate::{common, ethernets, modems, wifis, bridges, dummy_devices, bonds, nm_devices, tunnels, virtual_ethernets, vlans, vrfs};
use serde_with::skip_serializing_none;
use serde::{Serialize, Deserialize};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Network {
    #[serde(default, deserialize_with = "common::deserialize_version")]
    version: Option<u8>,
    #[serde(default, deserialize_with = "common::deserialize_renderer")]
    renderer: Option<String>,
    openvswitch: Option<OpenvSwitch>,
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Netplan {
    network: Network,
}
