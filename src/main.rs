#![allow(unused_imports)]
#![allow(unused_variables)]
use clap::{App, Arg, SubCommand};
use std::env;
use std::process;

use blue::Config;

fn main() {
    let matches = App::new("blue")
        .version("0.1.0")
        .author("Jack Hanslope <jackhansp@btinternet.com>")
        .about("A bluetooth cli written in rust")
        .subcommand(
            SubCommand::with_name("connect")
                .about("connect to a bluetooth device")
                .arg(
                    Arg::with_name("device")
                        .help("the device you'd like to connect to")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("disconnect")
                .about("disconnect from bluetooth devices")
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .help("disconnect from all connected bluetooth devices")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("device")
                        .help("the device you'd like to disconnect from")
                        .required(false)
                        .index(1)
                        .conflicts_with("all"),
                ),
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("connect") {
        println!("Connecting to {}", matches.value_of("device").unwrap());
    }

    if let Some(ref matches) = matches.subcommand_matches("disconnect") {
        if let Some(d) = matches.value_of("device") {
            println!("Disconnecting from {}", d);
        } else if matches.is_present("all") {
            println!("Disconnecting from all");
        } else {
            println!("No arguments provided, disconnecting from all.");
        }
    }

    // if let Err(e) = blue::run(config) {
    // println!("{}", e);
    // process::exit(1);
    // }
}
