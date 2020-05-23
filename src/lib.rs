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

struct Device {
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.command.as_str() {
        "connect" => println!("connecting"),
        "disconnect" => println!("disconnecting"),
        _ => println!("command not recognised"),
    }

    let list = adapter.get_device_list().unwrap();
    let speaker_path = list[1].clone();

    let speaker = Device::new(session, speaker_path);
    let speaker_name = speaker.get_name().unwrap();
    println!("speaker_name: {:?}", speaker_name);
    // match speaker.connect(2000) {
    // Ok(v) => println!("v: {:?}", v),
    // Err(e) => println!("e: {:?}", e),
    // }

    Ok(())
}

fn connect(device: String) {
    let session = &Session::create_session(None).unwrap();
    let adapter = Adapter::init(session)?;
}
