#![allow(unused_imports)]
#![allow(unused_variables)]
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::vec;

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_session::BluetoothSession as Session;

pub struct Config {
    pub command: String,
    pub device: String,
}

#[derive(Debug)]
pub struct Devi {
    path: String,
    alias: String,
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

fn read_devices() -> HashMap<String, String> {
    let contents = fs::read_to_string("device_store").unwrap();

    let mut store = HashMap::new();

    for line in contents.lines() {
        let line_vec: Vec<String> = line.split_whitespace().map(|x| x.to_string()).collect();
        store.insert(line_vec[0].clone(), line_vec[1].clone());
    }

    store
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let session = &Session::create_session(None).unwrap();
    let adapter = Adapter::init(session)?;
    println!("adapter successful");

    Ok(())
}

fn connect(device: String) {
    let store = read_devices();
}
