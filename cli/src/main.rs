extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hidapi;
#[macro_use]
extern crate error_chain;

mod cli;
mod device;
mod errors;

use clap::{App, Arg, SubCommand};
use device::Color;

fn main() {
    let matches = App::new("razer_test test")
        .version("0.0.1")
        .about("Tests razer devices on a very low level")
        .arg(
            Arg::with_name("debug")
                .short("D")
                .long("debug")
                .help("Enable debug"),
        ).subcommand(SubCommand::with_name("check").about("check if device is present"))
        .subcommand(SubCommand::with_name("listen").about("listen for key changes"))
        .subcommand(
            SubCommand::with_name("set-color")
                .about("set color")
                .arg(Arg::with_name("key").required(true))
                .arg(Arg::with_name("color").required(true)),
        ).get_matches();

    let mut log_builder = env_logger::Builder::from_default_env();

    if matches.is_present("debug") {
        log_builder.filter(None, log::LevelFilter::Debug);
    } else {
        log_builder.filter(None, log::LevelFilter::Info);
    }
    log_builder.init();

    if let Some(_) = matches.subcommand_matches("check") {
        cli::check().unwrap();
    } else if let Some(_) = matches.subcommand_matches("listen") {
        cli::listen().unwrap();
    } else if let Some(sub_matches) = matches.subcommand_matches("set-color") {
        let key_index =  sub_matches.value_of("key").unwrap().parse::<u8>().unwrap();
        let color = Color::parse(sub_matches.value_of("color").unwrap()).unwrap();
        cli::set_color(key_index, color).unwrap();
    }
}
