#[macro_use]
extern crate log;
extern crate api;
extern crate hex;

use crate::server::{Server, SetConfig, StartGrpcService};
use chrono::prelude::*;
use clap::{App, Arg};
use config::Config;
use env_logger::fmt::Color;
use env_logger::Builder;
use log::*;
use std::env;
use std::io::Write;
use tokio::signal;
use xactor::*;

mod server;
mod service;

const DEFAULT_GRPC_PORT: u32 = 6667;
const DEFAULT_HOST: &str = "[::1]";

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    init_logging();
    let mut config = get_default_config();
    let args = App::new("Spacemesh Multisig Message Server")
        .version("0.1.0")
        .author("Aviv Eyal <a@spacemesh.io>")
        .about("Provides a basic service for users to post and get multisig messages")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("FILE")
                .help("provide server configuration file")
                .takes_value(true),
        )
        .get_matches();

    if let Some(conf_file) = args.value_of("config") {
        config
            .merge(config::File::with_name(conf_file).required(false))
            .unwrap();
    }
    start_server(config).await?;

    // block app process until it is terminated
    signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c signal");

    info!("got signal - terminating server");
    Ok(())
}

async fn start_server(config: Config) -> Result<()> {
    // init the server with the provided config
    let server = Server::from_registry().await?;
    server.call(SetConfig(config.clone())).await??;

    info!("server starting...");

    server
        .call(StartGrpcService {
            port: config.get_int("port").unwrap() as u32,
            host: config.get_str("host").unwrap(),
        })
        .await??;

    info!("server running");
    Ok(())
}

fn init_logging() {
    let mut builder = Builder::new();

    builder
        .format_level(true)
        .format_timestamp(None)
        .format(move |buf, record| {
            let level_style = buf.default_level_style(record.level());

            let now: DateTime<Local> = Local::now();
            let date_format = format!("{}", now.to_rfc3339(),);

            let mut date_style = buf.style();
            date_style.set_color(Color::Yellow).set_bold(true);

            let mut file_name_style = buf.style();
            file_name_style.set_color(Color::Blue);

            let file_name = format!(
                "{} {}",
                record.file().unwrap().split('/').last().unwrap(),
                record.line().unwrap()
            );

            writeln!(
                buf,
                "{} {} {} {}",
                date_style.value(date_format),
                level_style.value(record.level()),
                file_name_style.value(file_name),
                record.args(),
            )
        })
        .filter(None, LevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
        builder.parse_filters(&env::var("RUST_LOG").unwrap());
    }

    builder.init();
}

fn get_default_config() -> config::Config {
    let mut config = Config::default();
    config
        .set_default("port", DEFAULT_GRPC_PORT.to_string())
        .unwrap()
        .set_default("host", DEFAULT_HOST)
        .unwrap()
        .clone()
}
