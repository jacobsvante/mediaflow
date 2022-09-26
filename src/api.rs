use std::sync::Arc;

use async_recursion::async_recursion;
use futures::future::try_join_all;
use reqwest::Client;
use serde::de::DeserializeOwned;
use tokio::sync::Mutex;

use crate::{
    config::Config,
    tokens::{BearerToken, TokenResponse},
};
use mediaflow_core::{
    FileId, FolderId, MediaflowFile, MediaflowFileDownload, MediaflowFolder, MediaflowFormat,
};

#[derive(Debug, Clone)]
pub struct RestApi {
    config: Config,
    client: reqwest::Client,
    token: Arc<Mutex<Option<BearerToken>>>,
}

impl RestApi {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: Client::new(),
            token: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn get_folders<T: MediaflowFolder + DeserializeOwned>(
        &self,
    ) -> crate::Result<Vec<T>> {
        let query = vec![Self::get_fields_query::<T>()];
        let resp = self.get_raw("folders", Some(query)).await?;
        let folders: Vec<T> = serde_json::from_str(&resp)?;
        Ok(folders)
    }

    pub async fn get_folder_children<T: MediaflowFolder + DeserializeOwned>(
        &self,
        folder_id: u32,
    ) -> crate::Result<Vec<T>> {
        let query = vec![Self::get_fields_query::<T>()];
        let resp = self
            .get_raw(format!("folders/{folder_id}/children"), Some(query))
            .await?;
        let folders: Vec<T> = serde_json::from_str(&resp)?;
        Ok(folders)
    }

    pub async fn get_folder_files<T: MediaflowFile + DeserializeOwned>(
        &self,
        folder_id: u32,
    ) -> crate::Result<Vec<T>> {
        let query = vec![Self::get_fields_query::<T>()];
        let resp = self
            .get_raw(format!("folders/{folder_id}/files"), Some(query))
            .await?;
        let files: Vec<T> = serde_json::from_str(&resp)?;
        Ok(files)
    }

    #[async_recursion]
    pub async fn get_folder_files_recursive<T: MediaflowFile + DeserializeOwned + Send>(
        &self,
        folder_id: u32,
    ) -> crate::Result<Vec<T>> {
        let mut files: Vec<T> = self.get_folder_files(folder_id).await?;
        let subfolders: Vec<FolderId> = self.get_folder_children(folder_id).await?;
        for subfolder in subfolders {
            files.extend(self.get_folder_files_recursive(subfolder.id).await?);
        }
        Ok(files)
    }

    pub(crate) async fn get_raw<T: ToString>(
        &self,
        endpoint: T,
        query: Option<Vec<(String, String)>>,
    ) -> crate::Result<String> {
        let access_token = self.access_token().await?;
        let url = self.config.base_url.join(&endpoint.to_string()).unwrap();
        let resp = self
            .client
            .request(reqwest::Method::GET, url)
            .header("Authorization", format!("Bearer {}", access_token))
            .query(&query.unwrap_or_default())
            .send()
            .await?;
        let status = resp.status().as_u16();
        let body = resp.text().await?;
        match serde_json::from_str::<crate::MediaFlowResponseError>(&body) {
            Ok(resp_error) => Err(crate::Error::ApiError(status, resp_error.error())),
            Err(_) => Ok(body),
        }
    }

    /// List all download formats
    pub async fn get_formats<T: MediaflowFormat + DeserializeOwned>(
        &self,
    ) -> crate::Result<Vec<T>> {
        let query = vec![Self::get_fields_query::<T>()];
        let resp = self.get_raw("format", Some(query)).await?;
        let files: Vec<T> = serde_json::from_str(&resp)?;
        Ok(files)
    }

    /// Get download links for the given file ID
    pub async fn get_file_downloads<T: MediaflowFileDownload + DeserializeOwned>(
        &self,
        file_id: u32,
    ) -> crate::Result<Vec<T>> {
        let query = vec![Self::get_fields_query::<T>()];
        let resp = self
            .get_raw(format!("files/{file_id}/downloads"), Some(query))
            .await?;
        let downloads: Vec<T> = serde_json::from_str(&resp)?;
        Ok(downloads)
    }

    /// Get original download link for the given file ID
    /// NOTE: This is an undocumented API endpoint, use with caution!
    /// (format ID 0 always "original" format)
    pub async fn get_file_download<T: MediaflowFileDownload + DeserializeOwned>(
        &self,
        file_id: u32,
        format_id: i32,
    ) -> crate::Result<T> {
        let query = vec![Self::get_fields_query::<T>()];
        let resp = self
            .get_raw(
                format!("files/{file_id}/downloads/{format_id}"),
                Some(query),
            )
            .await?;
        let mut downloads: Vec<T> = serde_json::from_str(&resp)?;
        if downloads.len() == 1 {
            Ok(downloads.pop().unwrap())
        } else {
            Err(crate::Error::UnexpectedApiResponseError(format!(
                "Expected 1 result for /files/{file_id}/downloads/{format_id}, got {0} results",
                downloads.len()
            )))
        }
    }

    /// Get all downloads of the given format for all files within the given folder
    pub async fn get_folder_file_download_list<
        T: MediaflowFileDownload + DeserializeOwned + Send,
    >(
        &self,
        folder_id: u32,
        format_id: i32,
    ) -> crate::Result<Vec<(FileId, T)>> {
        let chunk_size = self.config.max_concurrent_downloads as usize;
        let files = if recursive {
            log::info!("Recursively getting file downloads for folder {folder_id}...");
            self.get_folder_files_recursive::<FileId>(folder_id).await?
        } else {
            log::info!("Getting file downloads for folder {folder_id}...");
            self.get_folder_files::<FileId>(folder_id).await?
        };
        log::info!("Max concurrent downloads: {chunk_size}");
        let mut downloads: Vec<(FileId, T)> = Vec::with_capacity(files.len());
        for files_chunk in files.chunks(chunk_size) {
            let mut file_ids = Vec::with_capacity(chunk_size);
            let mut futures = Vec::with_capacity(chunk_size);
            for file in files_chunk {
                file_ids.push(FileId { id: file.id });
                futures.push(self.get_file_download::<T>(file.id, format_id));
            }
            let dwnlds = try_join_all(futures).await?;
            downloads.extend(file_ids.into_iter().zip(dwnlds));
        }
        Ok(downloads)
    }

    fn get_fields_query<T: DeserializeOwned>() -> (String, String) {
        let fields = crate::utils::serde_introspect::<T>();
        ("fields".into(), fields.join(","))
    }

    async fn access_token(&self) -> crate::Result<String> {
        let mut guard = self.token.lock().await;
        let token = if let Some(token) = &*guard {
            if token.close_to_expiring() {
                let token = self.authenticate().await?;
                token
            } else {
                token.clone()
            }
        } else {
            self.authenticate().await?
        };
        let access_token_string = token.access_token();
        *guard = Some(token);
        Ok(access_token_string)
    }

    async fn authenticate(&self) -> crate::Result<BearerToken> {
        let url = self.config.base_url.join("oauth2/token").unwrap();
        let resp = self
            .client
            .get(url)
            .query(&[
                ("grant_type", "password"),
                ("client_id", &self.config.client_id),
                ("client_secret", &self.config.client_secret),
                ("username", &self.config.username),
                ("password", &self.config.password),
            ])
            .send()
            .await?;
        let status = resp.status().as_u16();
        let body = resp.text().await?;
        match serde_json::from_str::<TokenResponse>(&body) {
            Ok(raw_token) => Ok(raw_token.into()),
            Err(_) => Err(crate::Error::AuthenticationError(status, body)),
        }
    }
}
