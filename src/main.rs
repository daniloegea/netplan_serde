use std::{env, fs};

use serde_yaml;

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
        println!("Pass a YAML file as parameter");
        return;
    }

    let yaml = fs::read_to_string(&args[1]).unwrap();

    let netplan: network::Netplan = match serde_yaml::from_str(&yaml) {
        Ok(n) => n,
        Err(err) => {
            println!("Error {:?}", err);
            return;
        }
    };

    println!("{netplan:#?}");

    let yaml = serde_yaml::to_string(&netplan).unwrap();
    println!("{yaml}");
}
