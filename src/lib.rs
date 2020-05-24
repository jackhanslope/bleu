#![allow(unused_imports)]
#![allow(unused_variables)]
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

fn read_devices() -> Vec<Devi> {
    let contents = fs::read_to_string("device_store").unwrap();
    let mut store: Vec<Devi> = Vec::new();
    for line in contents.lines() {
        let mut count = 0;
        let mut ent = Devi {
            path: String::from(""),
            alias: String::from(""),
        };
        for val in line.split_whitespace() {
            if count == 0 {
                ent.path = val.to_string();
            } else {
                ent.alias = val.to_string();
            }
            count += 1;
        }
        store.push(ent);
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
