use std::{env, fs};

use serde_yaml;

use glob::glob;

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
pub mod network;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Pass a root dir as parameter");
        return;
    }

    let directory_glob = format!("{}/etc/netplan/*.yaml", args[1]);
    let mut files = Vec::new();

    if let Ok(paths) = glob(&directory_glob) {
        for file in paths {
            if let Ok(path) = file {
                files.push(path);
            }
        }
    }

    let mut state = network::Netplan::default();

    for filename in &files {
        let yaml = match fs::read_to_string(filename) {
            Ok(data) => data,
            Err(err) => {
                println!("Error: {:?}: {err}", filename);
                return;
            }
        };

        let mut netplan: network::Netplan = match serde_yaml::from_str(&yaml) {
            Ok(n) => n,
            Err(err) => {
                println!("Error {:?}", err);
                return;
            }
        };
        
        // Merge renderer and version
        if let Some(ref renderer) = netplan.network.renderer {
            state.network.renderer = Some(renderer.to_string());
        }
        if let Some(version) = netplan.network.version {
            state.network.version = Some(version);
        }

        // Merge ethernets
        if let Some(ref mut ethernets) = netplan.network.ethernets {
            if state.network.ethernets.is_none() {
                state.network.ethernets = netplan.network.ethernets;
            } else {
                if let Some(ref mut current_ethernets) = state.network.ethernets {
                    current_ethernets.append(ethernets);
                }
            }
        }

        // Merge wifis
        if let Some(ref mut wifis) = netplan.network.wifis {
            if state.network.wifis.is_none() {
                state.network.wifis = netplan.network.wifis;
            } else {
                if let Some(ref mut current_wifis) = state.network.wifis {
                    current_wifis.append(wifis);
                }
            }
        }

        // Merge bonds
        if let Some(ref mut bonds) = netplan.network.bonds {
            if state.network.bonds.is_none() {
                state.network.bonds = netplan.network.bonds;
            } else {
                if let Some(ref mut current_bonds) = state.network.bonds {
                    current_bonds.append(bonds);
                }
            }
        }

        // Merge bridges
        if let Some(ref mut bridges) = netplan.network.bridges {
            if state.network.bridges.is_none() {
                state.network.bridges = netplan.network.bridges;
            } else {
                if let Some(ref mut current_bridges) = state.network.bridges {
                    current_bridges.append(bridges);
                }
            }
        }

        // Merge bridges
        if let Some(ref mut dummy_devices) = netplan.network.dummy_devices {
            if state.network.dummy_devices.is_none() {
                state.network.dummy_devices = netplan.network.dummy_devices;
            } else {
                if let Some(ref mut current_dummy_devices) = state.network.dummy_devices {
                    current_dummy_devices.append(dummy_devices);
                }
            }
        }

        // Merge modems
        if let Some(ref mut modems) = netplan.network.modems {
            if state.network.modems.is_none() {
                state.network.modems = netplan.network.modems;
            } else {
                if let Some(ref mut current_modems) = state.network.modems {
                    current_modems.append(modems);
                }
            }
        }

        // Merge nm_devices
        if let Some(ref mut nm_devices) = netplan.network.nm_devices {
            if state.network.nm_devices.is_none() {
                state.network.nm_devices = netplan.network.nm_devices;
            } else {
                if let Some(ref mut current_nm_devices) = state.network.nm_devices {
                    current_nm_devices.append(nm_devices);
                }
            }
        }

        // Merge tunnels
        if let Some(ref mut tunnels) = netplan.network.tunnels {
            if state.network.tunnels.is_none() {
                state.network.tunnels = netplan.network.tunnels;
            } else {
                if let Some(ref mut current_tunnels) = state.network.tunnels {
                    current_tunnels.append(tunnels);
                }
            }
        }

        // Merge virtual_ethernets
        if let Some(ref mut virtual_ethernets) = netplan.network.virtual_ethernets {
            if state.network.virtual_ethernets.is_none() {
                state.network.virtual_ethernets = netplan.network.virtual_ethernets;
            } else {
                if let Some(ref mut current_virtual_ethernets) = state.network.virtual_ethernets {
                    current_virtual_ethernets.append(virtual_ethernets);
                }
            }
        }

        // Merge vlans
        if let Some(ref mut vlans) = netplan.network.vlans {
            if state.network.vlans.is_none() {
                state.network.vlans = netplan.network.vlans;
            } else {
                if let Some(ref mut current_vlans) = state.network.vlans {
                    current_vlans.append(vlans);
                }
            }
        }

        // Merge vrfs
        if let Some(ref mut vrfs) = netplan.network.vrfs {
            if state.network.vrfs.is_none() {
                state.network.vrfs = netplan.network.vrfs;
            } else {
                if let Some(ref mut current_vrfs) = state.network.vrfs {
                    current_vrfs.append(vrfs);
                }
            }
        }
    }

    let yaml = serde_yaml::to_string(&state).unwrap();
    println!("{yaml}");
}
