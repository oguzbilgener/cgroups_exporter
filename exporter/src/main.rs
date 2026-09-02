#![allow(clippy::multiple_crate_versions)]
//! # Cgroups Exporter
use cgroups_exporter_config::load_config;
use clap::Parser;
use envconfig::Envconfig;
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

use crate::{
    file_watcher::watch_config_file,
    logging::{LogFormat, LogLevel, set_panic_hook, setup_logging},
    matcher::MatchableConfig,
    server::SharedConfig,
    shell::ShellEvaluator,
};

mod cgroups;
mod file_watcher;
mod logging;
mod matcher;
mod metadata;
#[allow(clippy::ref_option)]
mod procs;
mod render;
mod server;
mod shell;

const DEFAULT_LISTEN_ADDR: &str = "127.0.0.1:9753";

#[derive(Parser)]
#[command(version, about = "A Prometheus exporter for cgroups and processes", long_about = None)]
struct Cli {
    /// Path to the config file.
    #[arg(short, long)]
    config: Option<PathBuf>,
    /// The address to listen on.
    #[arg(short, long)]
    listen_addr: Option<String>,
    /// If provided, test the config file and exit.
    #[arg(short, long)]
    test: bool,
    /// If provided, watch the config file for changes and reload the config.
    #[arg(short, long)]
    watch: bool,
}

#[derive(Envconfig)]
struct EnvConfig {
    /// The log level.
    #[envconfig(from = "LOG_LEVEL")]
    log_level: Option<LogLevel>,
    #[envconfig(from = "LOG_FORMAT")]
    log_format: Option<LogFormat>,

    /// Path to the config file.
    #[envconfig(from = "CONFIG_PATH")]
    config_path: Option<PathBuf>,
    /// The address to listen on.
    #[envconfig(from = "LISTEN_ADDR")]
    listen_addr: Option<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let env_config = EnvConfig::init_from_env().expect("Failed to load env config");
    setup_logging(env_config.log_level, env_config.log_format).expect("Failed to set up logging");
    set_panic_hook();
    let cli = Cli::parse();
    let config_path = config_path(&cli, &env_config).expect("No config path provided");
    let listen_addr = listen_addr(&cli, &env_config);

    let config_path = tokio::fs::canonicalize(config_path)
        .await
        .expect("Failed to canonicalize config path");
    let config = load_config(&config_path)
        .await
        .expect("Failed to load config");
    if cli.test {
        info!("Config file is valid");
        return;
    }

    info!(
        config_path =% config_path.display(),
        cgroups =% config.cgroups.len(),
        processes =% config.processes.len(),
        "Loaded config file"
    );

    let config: MatchableConfig = config.try_into().expect("Failed to parse config file");
    let evaluator = ShellEvaluator::new(config.shell_commands.cache_size);
    let config = SharedConfig::new(config);

    if cli.watch {
        info!(path = % config_path.display(), "Watching config file for changes");
        tokio::spawn(watch_config_file(config_path, config.clone()));
    }

    let cancel_token = CancellationToken::new();

    tokio::select! {
        res = server::serve(listen_addr, config, evaluator, cancelled(cancel_token.clone())) => {
            if let Err(err) = res {
                error!(%err, "Server failed");
            }
        }
        () = shutdown_signal() => {
            info!("Received shutdown signal");
            cancel_token.cancel();
        }
    }
    info!("Server shut down");
}

fn listen_addr<'main>(cli: &'main Cli, env_config: &'main EnvConfig) -> SocketAddr {
    cli.listen_addr
        .as_deref()
        .and_then(|addr| {
            addr.parse()
                .map_err(|err| {
                    error!(%err, "Failed to parse listen address from command line arguments, trying env config or default");
                })
                .ok()
        })
        .or_else(|| {
            env_config
                .listen_addr
                .as_deref()
                .and_then(|addr| addr.parse().map_err(|err| {
                    error!(%err, "Failed to parse listen address from the environment variable, falling back to default");
                }).ok())
        })
        .unwrap_or_else(|| DEFAULT_LISTEN_ADDR.parse().unwrap())
}

fn config_path<'main>(cli: &'main Cli, env_config: &'main EnvConfig) -> Option<&'main Path> {
    cli.config.as_deref().or(env_config.config_path.as_deref())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
}

/// Converts `cancel_token.cancelled()` into a `'static` future.
async fn cancelled(cancel_token: CancellationToken) {
    cancel_token.cancelled().await;
}
