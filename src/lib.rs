use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;

use bimap::BiHashMap;
use clap::App;
use serde_json;

use serde::{Deserialize, Serialize};

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_session::BluetoothSession as Session;

use dirs;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    devices: BiHashMap<String, String>,
}

fn read_devices() -> Result<BiHashMap<String, String>, Box<dyn Error>> {
    let mut path = dirs::config_dir().unwrap();
    path.push("bleu/config.json");
    let json_file = match File::open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                return Err("Error accessing stored devices: config file does not exist.".into())
            }
            _ => return Err(
                "Error accessing stored devices: 'device_store.json' exists but can't be opened."
                    .into(),
            ),
        },
    };

    let config: Config = serde_json::from_reader(json_file)?;
    let store = config.devices;
    Ok(store)
}

pub fn run(app: App) -> Result<(), Box<dyn Error>> {
    let matches = app.get_matches();
    if let Some(ref matches) = matches.subcommand_matches("connect") {
        // can do straight unwrap becuase it'll always have a value
        let device = matches.value_of("device").unwrap();

        connect(device.to_string())?;
    }

    if let Some(ref matches) = matches.subcommand_matches("disconnect") {
        if let Some(d) = matches.value_of("device") {
            disconnect_single(d.to_string())?;
        } else if matches.is_present("all") {
            disconnect_all()?;
        }
    }

    if let Some(_) = matches.subcommand_matches("connected") {
        list_connected()?;
    }

    Ok(())
}

fn connect(alias: String) -> Result<(), Box<dyn Error>> {
    let session = &Session::create_session(None)?;
    let store = read_devices()?;

    let path = match store.get_by_left(&alias) {
        Some(path) => path,
        None => return Err(format!("No entry found in the device store for '{}'", alias).into()),
    };

    let device = Device::new(session, path.to_string());
    if device.is_connected()? {
        // TODO: change this into a return error
        println!("Already connected to {}", alias);
    } else {
        println!("Attempting to connect to {}", alias);
        device.connect(10000).ok();
        if device.is_connected()? {
            println!("Connection to {} successful", alias);
        } else {
            return Err("Connection unsuccessful".into());
        }
    }

    Ok(())
}

fn disconnect_all() -> Result<(), Box<dyn Error>> {
    println!("Disconnecting from all");
    let session = &Session::create_session(None)?;
    let adapter = Adapter::init(session)?;
    let devices = adapter.get_device_list()?;

    let mut connected_devices = 0;
    for device in devices {
        let device = Device::new(session, device);
        if device.is_connected()? {
            connected_devices += 1;
            // TODO: get friendlier alias from device store?
            println!("Disconnecting from {}.", device.get_alias()?);
            device.disconnect()?;
            println!("Successfullly disconnected from {}.", device.get_alias()?);
        }
    }

    if connected_devices == 0 {
        return Err("No connected devices.".into());
    }

    Ok(())
}

fn disconnect_single(alias: String) -> Result<(), Box<dyn Error>> {
    let session = &Session::create_session(None)?;
    let store = read_devices()?;
    let path = match store.get_by_left(&alias) {
        Some(path) => path,
        None => return Err(format!("No entry found in the device store for '{}'", alias).into()),
    };
    let device = Device::new(session, path.to_string());
    if !device.is_connected()? {
        return Err(format!("Not connected to {}", alias).into());
    } else {
        println!("Attempting to disconnect to {}", alias);
        device.disconnect()?;
        if !device.is_connected()? {
            println!("Disconnection from {} successful", alias);
        } else {
            return Err("Disconnection unsuccessful".into());
        }
    }
    Ok(())
}

fn list_connected() -> Result<(), Box<dyn Error>> {
    let session = &Session::create_session(None)?;
    let adapter = Adapter::init(session)?;

    let store = read_devices()?;
    let mut connected_devices = Vec::new();

    for device_path in adapter.get_device_list()? {
        let device = Device::new(session, device_path.clone());
        if device.is_connected()? {
            match store.get_by_right(&device_path) {
                Some(alias) => connected_devices.push(String::from(alias)),
                None => {
                    let device = Device::new(session, device_path);
                    let name = device.get_name()?;
                    let prefix = String::from("Unknown device: ");
                    connected_devices.push(format!("{}{}", prefix, name));
                }
            }
        };
    }

    if connected_devices.len() == 0 {
        println!("No connected devices");
    } else {
        for device in connected_devices {
            println!("{}", device);
        }
    }

    Ok(())
}
