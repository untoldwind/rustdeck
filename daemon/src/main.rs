extern crate clap;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate hidapi;
#[macro_use]
extern crate error_chain;
extern crate dbus;

mod daemon;
mod dbus_server;
mod device;
mod errors;

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

    let daemon: daemon::Daemon = Default::default();

    daemon.run().expect("Unable to start daemon loop");
}
