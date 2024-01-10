use std::{net::IpAddr, str::FromStr};

use clap::Parser;
use http_server::{Config, Server};

use crate::cli::Cli;

mod cli;

fn main() {
    let cli = Cli::parse();

    let mut server = Server::new(Config {
        address: IpAddr::from_str(&cli.address).unwrap(),
        port: cli.port,
        default_route: Some(cli.default_route),
        thread_count: cli.thread_count,
    });

    println!("server started.");

    server.start().unwrap();
}
