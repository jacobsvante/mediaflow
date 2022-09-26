use std::{
    io::{stdout, Write},
    os::unix::prelude::OsStrExt,
};

use clap::Parser;
use log::{debug, LevelFilter};
use serde::Serialize;

use super::opts::RawRestApiSubCommand;
use super::{helpers::safe_extract_arg, ini};
use super::{
    opts::{Opts, RestApiSubCommand, SubCommand},
    CliResult,
};
use crate::api::RestApi;
use crate::{config::Config, Result};
use mediaflow_core::{FileBase, FileDownloadFull, FileFull, FolderFull, FormatFull};

pub async fn run() -> CliResult {
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
            max_concurrent_downloads,
        } => {
            let config = Config::new(
                client_id,
                client_secret,
                username,
                password,
                Some(max_concurrent_downloads),
            );
            let api = RestApi::new(config);
            rest_api_sub_command(subcmd, &api).await?;
        }
        SubCommand::DefaultIniPath => {
            ini::default_location().map(|p| stdout().write(p.as_os_str().as_bytes()));
        }
    }

    Ok(())
}

async fn rest_api_sub_command(subcmd: RestApiSubCommand, api: &RestApi) -> CliResult {
    match subcmd {
        RestApiSubCommand::Folders => {
            pretty_print_json(&api.get_folders::<FolderFull>().await?)?;
        }
        RestApiSubCommand::FolderChildren { folder_id } => {
            pretty_print_json(&api.get_folder_children::<FolderFull>(folder_id).await?)?;
        }
        RestApiSubCommand::FolderFiles {
            folder_id,
            full,
            recursive,
        } => {
            if full {
                let files = if recursive {
                    api.get_folder_files_recursive::<FileFull>(folder_id)
                        .await?
                } else {
                    api.get_folder_files::<FileFull>(folder_id).await?
                };
                pretty_print_json(&files)?
            } else {
                let files = if recursive {
                    api.get_folder_files_recursive::<FileBase>(folder_id)
                        .await?
                } else {
                    api.get_folder_files::<FileBase>(folder_id).await?
                };
                pretty_print_json(&files)?
            };
        }
        RestApiSubCommand::Formats => {
            pretty_print_json(&api.get_formats::<FormatFull>().await?)?;
        }
        RestApiSubCommand::FileDownloads { file_id } => {
            pretty_print_json(&api.get_file_downloads::<FileDownloadFull>(file_id).await?)?;
        }
        RestApiSubCommand::FileDownload { file_id, format_id } => {
            pretty_print_json(
                &api.get_file_download::<FileDownloadFull>(file_id, format_id)
                    .await?,
            )?;
        }
        RestApiSubCommand::FolderDownloads {
            folder_id,
            format_id,
            recursive,
        } => {
            pretty_print_json(
                &api.get_folder_file_download_list::<FileDownloadFull>(
                    folder_id, format_id, recursive,
                )
                .await?,
            )?;
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

fn pretty_print_json<T: Serialize>(serializable: &T) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(&serializable)?);
    Ok(())
}
