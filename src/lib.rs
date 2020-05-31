#![allow(unused_imports)]
#![allow(unused_variables)]
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::path::Path;
use std::vec;

use clap::{App, Arg, SubCommand};

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_session::BluetoothSession as Session;

use serde_json;

fn read_devices() -> Result<HashMap<String, String>, Box<dyn Error>> {
    let json_file_path = Path::new("device_store.json");
    let json_file = match File::open(json_file_path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                return Err(
                    "Error accessing stored devices: 'device_store.json' does not exist.".into(),
                )
            }
            _ => return Err(
                "Error accessing stored devices: 'device_store.json' exists but can't be opened."
                    .into(),
            ),
        },
    };

    let store = serde_json::from_reader(json_file)?;
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
            //TODO: implement disconnect from single
            // println!("Disconnecting from {}", d);
            println!("Disconnect from single not implimented yet.");
        } else if matches.is_present("all") {
            disconnect_all()?;
        } else {
            println!("please provide an arg");
        }
    }

    Ok(())
}

fn connect(alias: String) -> Result<(), Box<dyn Error>> {
    let session = &Session::create_session(None)?;
    let adapter = Adapter::init(session)?;
    let store = read_devices()?;

    let path = match store.get(&alias) {
        Some(path) => path,
        None => return Err(format!("No entry found in the device store for '{}'", alias).into()),
    };

    let device = Device::new(session, path.to_string());
    if device.is_connected()? {
        println!("Already connected to {}", alias);
    } else {
        println!("Attempting to connect to {}", alias);
        device.connect(10000).ok();
        //FIXME: sometimes the line below is thinking that we've connected even when we haven't
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
