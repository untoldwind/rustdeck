
extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hidapi;
#[macro_use]
extern crate error_chain;

mod errors;
mod device;
mod daemon_loop;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("rustdeck-daemon")
        .version("0.0.1")
        .about("Elgato StreamDeck dbus daemon")
        .arg(
            Arg::with_name("debug")
                .short("D")
                .long("debug")
                .help("Enable debug"),
        ).get_matches();

    let mut log_builder = env_logger::Builder::from_default_env();

    if matches.is_present("debug") {
        log_builder.filter(None, log::LevelFilter::Debug);
    } else {
        log_builder.filter(None, log::LevelFilter::Info);
    }
    log_builder.init();
   
}