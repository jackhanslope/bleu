#![allow(unused_imports)]
#![allow(unused_variables)]
use std::error::Error;

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let session = &Session::create_session(None).unwrap();
    let adapter = Adapter::init(session)?;

    println!("adapter successful");

    Ok(())
}
