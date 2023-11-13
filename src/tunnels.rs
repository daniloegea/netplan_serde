use std::fmt::Debug;

use serde::{Serialize, Deserialize};

use crate::common;
use crate::common::NetworkManager;
use crate::common_physical::OpenvSwitch;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tunnel {
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

    // Tunnels options
    mode: Option<String>,
    local: Option<String>,
    remote: Option<String>,
    ttl: Option<u64>,
    #[serde(alias = "keys", default)]
    key: Option<TunnelKey>,

    // Wireguard options
    mark: Option<u64>,
    port: Option<u64>,
    peers: Option<Vec<WireguardPeer>>,

    // Vxlan
    link: Option<String>,
    #[serde(alias = "aging")]
    ageing: Option<u64>,
    id: Option<u64>,
    limit: Option<u64>,
    #[serde(rename = "type-of-service")]
    type_of_service: Option<u64>,
    #[serde(rename = "flow-label")]
    flow_label: Option<u64>,
    #[serde(default, rename = "do-not-fragment", deserialize_with = "common::deserialize_boolean")]
    do_not_fragment: Option<String>,
    #[serde(default, rename = "short-circuit", deserialize_with = "common::deserialize_boolean")]
    short_circuit: Option<String>,
    #[serde(default, rename = "arp-proxy", deserialize_with = "common::deserialize_boolean")]
    arp_proxy: Option<String>,
    #[serde(default, rename = "mac-learning", deserialize_with = "common::deserialize_boolean")]
    mac_learning: Option<String>,
    notifications: Option<Vec<String>>,
    checksums: Option<Vec<String>>,
    extensions: Option<Vec<String>>,
    #[serde(default, rename = "port-range")]
    port_range: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TunnelKey {
    Key(String),
    Keys(TunnelKeyOptions),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TunnelKeyOptions {
    input: Option<String>,
    output: Option<String>,
    private: Option<String>,
    #[serde(rename = "private-key-flags", default)]
    private_key_flags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WireguardPeer {
    keys: Option<WireguardKey>,
    keepalive: Option<u64>,
    endpoint: Option<String>,
    #[serde(rename = "allowed-ips", default)]
    allowed_ips: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WireguardKey {
    public: Option<String>,
    shared: Option<String>,
} 
