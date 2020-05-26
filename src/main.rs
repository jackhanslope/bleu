#![allow(unused_imports)]
#![allow(unused_variables)]
use std::env;
use std::process;

use blue::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        process::exit(1);
    });

    if let Err(e) = blue::run(config) {
        println!("{}", e);
        process::exit(1);
    }
}
