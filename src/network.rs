use std::{collections::BTreeMap, fmt::Debug};

use crate::common_physical::OpenvSwitch;
use crate::{common, ethernets, modems, wifis, bridges, dummy_devices, bonds, nm_devices, tunnels, virtual_ethernets, vlans, vrfs};
use serde_with::skip_serializing_none;
use serde::{Serialize, Deserialize};

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Network {
    #[serde(default, deserialize_with = "common::deserialize_version")]
    pub version: Option<u8>,
    #[serde(default, deserialize_with = "common::deserialize_renderer")]
    pub renderer: Option<String>,
    pub openvswitch: Option<OpenvSwitch>,
    pub ethernets: Option<BTreeMap<String, ethernets::Ethernet>>,
    pub modems: Option<BTreeMap<String, modems::Modem>>,
    pub wifis: Option<BTreeMap<String, wifis::Wifi>>,
    pub bridges: Option<BTreeMap<String, bridges::Bridge>>,
    #[serde(rename = "dummy-devices")]
    pub dummy_devices: Option<BTreeMap<String, dummy_devices::DummyDevice>>,
    pub bonds: Option<BTreeMap<String, bonds::Bond>>,
    pub tunnels: Option<BTreeMap<String, tunnels::Tunnel>>,
    #[serde(rename = "virtual-ethernets")]
    pub virtual_ethernets: Option<BTreeMap<String, virtual_ethernets::VirtualEthernet>>,
    pub vlans: Option<BTreeMap<String, vlans::Vlan>>,
    pub vrfs: Option<BTreeMap<String, vrfs::Vrf>>,
    #[serde(rename = "nm-devices")]
    pub nm_devices: Option<BTreeMap<String, nm_devices::NmDevice>>,
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Netplan {
    pub network: Network,
}
