use std::io;
use clap::ArgMatches;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, Layer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init(matches: &ArgMatches) {
    let level = match matches.get_count("verbosity") {
        1 => LevelFilter::DEBUG,
        2 => LevelFilter::TRACE,
         _ => LevelFilter::INFO,
    };
    let filter = EnvFilter::builder()
        .with_default_directive(level.into())
        .from_env_lossy();
    let layer = tracing_subscriber::fmt::layer()
        .with_writer(io::stderr)
        .with_filter(filter);
    tracing_subscriber::registry()
        .with(layer)
        .init();
}