use clap::Parser;

use linuxftools::{signal, commands};

fn main() {
    env_logger::init();
    signal::setup_signal_handler();

    let cmd = commands::Fallocate::parse();
    match cmd.run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}