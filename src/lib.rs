#![allow(unused_imports)]
#![allow(unused_variables)]
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::ErrorKind;
use std::vec;

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_session::BluetoothSession as Session;

pub struct Config {
    pub command: String,
    pub device: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let command = args[1].clone();
        let device = args[2].clone();

        Ok(Config { command, device })
    }
}

fn read_devices() -> Result<HashMap<String, String>, &'static str> {
    let contents =
        match fs::read_to_string("device_store") {
            Ok(file) => file,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    return Err("Error accessing stored devices: 'device_store' does not exist.")
                }
                _ => return Err(
                    "Error accessing stored devices: 'device_store' exists but can't be opened.",
                ),
            },
        };

    let mut store = HashMap::new();

    for line in contents.lines() {
        let line_vec: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
        store.insert(line_vec[0].clone(), line_vec[1].clone());
    }

    Ok(store)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let "connect" = &config.command[..] {
        connect(config.device)?;
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
        if device.is_connected()? {
            println!("Connection to {} successful", alias);
        } else {
            return Err("Connection unsuccessful".into());
        }
    }

    Ok(())
}
