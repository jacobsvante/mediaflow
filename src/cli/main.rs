use std::{
    io::{stdout, Write},
    os::unix::prelude::OsStrExt,
};

use clap::Parser;
use log::{debug, LevelFilter};

use super::opts::{Opts, RestApiSubCommand, SubCommand};
use super::{error::CliError, opts::RawRestApiSubCommand};
use super::{helpers::safe_extract_arg, ini};
use crate::config::Config;
use crate::{
    api::RestApi,
    entities::{FileBase, FileFull, FolderFull},
};

pub async fn run() -> Result<(), CliError> {
    if let Some(level_filter) = safe_extract_arg::<LevelFilter>("level-filter") {
        env_logger::builder().filter(None, level_filter).init();
    }

    if let Err(err) = ini::to_env() {
        debug!("Couldn't load INI: {}", err);
    };

    let cli_opts = Opts::parse();

    match cli_opts.subcmd {
        SubCommand::RestApi {
            client_id,
            client_secret,
            username,
            password,
            subcmd,
        } => {
            let config = Config::new(client_id, client_secret, username, password);
            let mut api = RestApi::new(config);
            rest_api_sub_command(subcmd, &mut api).await?;
        }
        SubCommand::DefaultIniPath => {
            ini::default_location().map(|p| stdout().write(p.as_os_str().as_bytes()));
        }
    }

    Ok(())
}

async fn rest_api_sub_command(
    subcmd: RestApiSubCommand,
    api: &mut RestApi,
) -> Result<(), CliError> {
    match subcmd {
        RestApiSubCommand::Folders => {
            let folders = api.get_folders::<FolderFull>().await?;
            println!("{}", serde_json::to_string_pretty(&folders)?);
        }
        RestApiSubCommand::FolderChildren { folder_id } => {
            let folders = api.get_folder_children::<FolderFull>(folder_id).await?;
            println!("{}", serde_json::to_string_pretty(&folders)?);
        }
        RestApiSubCommand::FolderFiles {
            folder_id,
            full,
            recursive,
        } => {
            let output = if full {
                let files = if recursive {
                    api.get_folder_files_recursive::<FileFull>(folder_id)
                        .await?
                } else {
                    api.get_folder_files::<FileFull>(folder_id).await?
                };
                serde_json::to_string_pretty(&files)?
            } else {
                let files = if recursive {
                    api.get_folder_files_recursive::<FileBase>(folder_id)
                        .await?
                } else {
                    api.get_folder_files::<FileBase>(folder_id).await?
                };
                serde_json::to_string_pretty(&files)?
            };
            println!("{output}");
        }
        RestApiSubCommand::Raw { subcmd } => match subcmd {
            RawRestApiSubCommand::Get { endpoint, query } => {
                let body = api.get_raw(endpoint, Some(query)).await?;
                println!("{body}");
            }
        },
    };
    Ok(())
}
