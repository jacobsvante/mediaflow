use std::path::PathBuf;

use clap::Parser;
use log::LevelFilter;

use super::env::EnvVar;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
#[clap(name = "mediaflow", version = VERSION)]
pub(crate) struct Opts {
    #[clap(short = 's', long = "ini-section", env, default_value = "mediaflow")]
    ini_section: String,
    /// Where to load INI from, defaults to your OS's config directory.
    #[clap(short = 'p', long = "ini-path", env)]
    ini_path: Option<PathBuf>,
    #[clap(subcommand)]
    pub(crate) subcmd: SubCommand,
    /// Set the log level
    #[clap(
        short = 'l',
        long = "log-level",
        value_name = "level",
        env = "LOG_LEVEL"
    )]
    level_filter: Option<LevelFilter>,
}

#[derive(Debug, Parser)]
pub(crate) enum SubCommand {
    #[clap(name = "rest-api")]
    RestApi {
        #[clap(short = 'i', long, env = EnvVar::ClientId.into())]
        client_id: String,
        #[clap(short = 's', long, env = EnvVar::ClientSecret.into())]
        client_secret: String,
        #[clap(short = 'u', long, env = EnvVar::Username.into())]
        username: String,
        #[clap(short = 'p', long, env = EnvVar::Password.into())]
        password: String,
        #[clap(subcommand)]
        subcmd: RestApiSubCommand,
    },
    #[clap(name = "default-ini-path")]
    DefaultIniPath,
}

#[derive(Debug, Parser)]
pub(crate) enum RestApiSubCommand {
    Folders,
    FolderChildren {
        folder_id: u32,
    },
    FolderFiles {
        folder_id: u32,
        #[clap(short = 'F', long)]
        full: bool,
        #[clap(short = 'r', long)]
        recursive: bool,
    },
    Raw {
        #[clap(subcommand)]
        subcmd: RawRestApiSubCommand,
    },
}

#[derive(Debug, Parser)]
pub(crate) enum RawRestApiSubCommand {
    #[clap(name = "get")]
    Get {
        /// The endpoint to get data for
        endpoint: String,
        #[clap(short = 'q', long = "query", parse(try_from_str=two_tuple_on_equal_sign))]
        query: Vec<(String, String)>,
    },
}

fn two_tuple_on_equal_sign(s: &str) -> Result<(String, String), &'static str> {
    match s.split_once('=') {
        Some((key, val)) => Ok((key.into(), val.into())),
        None => Err("Must be equal sign delimited"),
    }
}
