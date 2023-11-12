
use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::common;
use crate::common_physical;
use crate::common_physical::OpenvSwitch;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ethernet {

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

    // Options for physical devices
    #[serde(rename = "match", default)]
    matching: Option<common_physical::Match>,
    #[serde(rename = "set-name", default)]
    set_name: Option<String>,
    #[serde(default, deserialize_with = "common::deserialize_boolean")]
    wakeonlan: Option<String>,
    #[serde(rename = "emit-lldp", default, deserialize_with = "common::deserialize_boolean")]
    emit_lldp: Option<String>,
    #[serde(rename = "receive-checksum-offload", default, deserialize_with = "common::deserialize_boolean")]
    receive_checksum_offload: Option<String>,
    #[serde(rename = "transmit-checksum-offload", default, deserialize_with = "common::deserialize_boolean")]
    transmit_checksum_offload: Option<String>,
    #[serde(rename = "tcp-segmentation-offload", default, deserialize_with = "common::deserialize_boolean")]
    tcp_segmentation_offload: Option<String>,
    #[serde(rename = "tcp6-segmentation-offload", default, deserialize_with = "common::deserialize_boolean")]
    tcp6_segmentation_offload: Option<String>,
    #[serde(rename = "generic-segmentation-offload", default, deserialize_with = "common::deserialize_boolean")]
    generic_segmentation_offload: Option<String>,
    #[serde(rename = "generic-receive-offload", default, deserialize_with = "common::deserialize_boolean")]
    generic_receive_offload: Option<String>,
    #[serde(rename = "large-receive-offload", default, deserialize_with = "common::deserialize_boolean")]
    large_receive_offload: Option<String>,
    openvswitch: Option<OpenvSwitch>,
}


