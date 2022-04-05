use clap::{App, Arg, SubCommand};
use std::process;

fn main() {
    let app = App::new("bleu")
        .version(clap::crate_version!())
        .author("Jack Hanslope <code@jackhanslope.com>")
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
                        .required(true)
                        .index(1)
                        .conflicts_with("all"),
                ),
        )
        .subcommand(SubCommand::with_name("connected").about("list currently connected devices"));

    if let Err(e) = bleu::run(app) {
        println!("{}", e);
        process::exit(1);
    }
}
