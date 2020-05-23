use std::env;
use std::process;

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_session::BluetoothSession as Session;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });

    println!("Command: {}", config.command);
    println!("Device: {}", config.device);

    let session = &Session::create_session(None).unwrap();
    let adapter: Adapter = Adapter::init(session).unwrap();
    let device: Device = adapter.get_first_device().unwrap();
    println!("device: {:?}", device);
}

struct Config {
    command: String,
    device: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let command = args[1].clone();
        let device = args[2].clone();

        Ok(Config { command, device })
    }
}
