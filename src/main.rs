use std::env;

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_session::BluetoothSession as Session;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let device = &args[2];

    println!("Running {}", command);
    println!("Device: {}", device);

    let session = &Session::create_session(None).unwrap();
    let adapter: Adapter = Adapter::init(session).unwrap();
    let device: Device = adapter.get_first_device().unwrap();
    println!("device: {:?}", device);
}
