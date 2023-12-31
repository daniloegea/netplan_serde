use std::fmt::Debug;

use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

use crate::common;
use crate::common::NetworkManager;
use crate::common_physical::OpenvSwitch;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Vrf {
    // Common options
    #[serde(deserialize_with = "common::deserialize_renderer", default)]
    renderer: Option<String>,
    #[serde(deserialize_with = "common::deserialize_boolean", default)]
    dhcp4: Option<String>,
    #[serde(deserialize_with = "common::deserialize_boolean", default)]
    dhcp6: Option<String>,
    #[serde(rename = "ipv6-mtu", default)]
    ipv6_mtu: Option<u64>,
    #[serde(rename = "ipv6-privacy", default, deserialize_with = "common::deserialize_boolean")]
    ipv6_privacy: Option<String>,
    #[serde(rename = "link-local", default, deserialize_with = "common::deserialize_link_local")]
    link_local: Option<Vec<String>>,
    #[serde(rename = "ignore-carrier", default, deserialize_with = "common::deserialize_boolean")]
    ignore_carrier: Option<String>,
    #[serde(default, deserialize_with = "common::deserialize_boolean")]
    critical: Option<String>,
    #[serde(rename = "dhcp-identifier", default, deserialize_with = "common::deserialize_dhcp_identifier")]
    dhcp_identifier: Option<String>,
    #[serde(rename = "dhcp4-overrides", default)]
    dhcp4_overrides: Option<common::DHCPOverrides>,
    #[serde(rename = "dhcp6-overrides", default)]
    dhcp6_overrides: Option<common::DHCPOverrides>,
    #[serde(rename = "accept-ra", default, deserialize_with = "common::deserialize_boolean")]
    accept_ra: Option<String>,
    addresses: Option<Vec<common::IPAddress>>,
    #[serde(rename = "ipv6-address-generation", default, deserialize_with = "common::deserialize_ipv6_address_generation")]
    ipv6_address_generation: Option<String>,
    #[serde(rename = "ipv6-address-token", default)]
    ipv6_address_token: Option<String>,
    gateway4: Option<String>,
    gateway6: Option<String>,
    nameservers: Option<common::Nameservers>,
    #[serde(deserialize_with = "common::deserialize_macaddress", default)]
    macaddress: Option<String>,
    mtu: Option<u64>,
    #[serde(default, deserialize_with = "common::deserialize_boolean")]
    optional: Option<String>,
    #[serde(rename = "optional-addresses", default, deserialize_with = "common::deserialize_optional_addresses")]
    optional_addresses: Option<Vec<String>>,
    #[serde(rename = "activation-mode", deserialize_with = "common::deserialize_activation_mode", default)]
    activation_mode: Option<String>,
    routes: Option<Vec<common::Route>>,
    #[serde(rename = "routing-policy", default)]
    routing_policy: Option<Vec<common::RoutingPolicy>>,
    #[serde(rename = "neigh-suppress", default, deserialize_with = "common::deserialize_boolean")]
    neigh_suppress: Option<String>,

    // Backend options
    openvswitch: Option<OpenvSwitch>,
    networkmanager: Option<NetworkManager>,

    // VRFs options
    interfaces: Option<Vec<String>>,
    table: Option<u64>,
}
