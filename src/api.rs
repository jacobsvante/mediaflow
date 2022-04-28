use async_recursion::async_recursion;
use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{
    config::Config,
    entities::{BearerToken, FolderId, TokenResponse},
    MediaflowFile, MediaflowFolder,
};

#[derive(Debug)]
pub struct RestApi {
    config: Config,
    client: reqwest::Client,
    token: Option<BearerToken>,
}

impl RestApi {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: Client::new(),
            token: None,
        }
    }

    pub(crate) async fn get_raw<T: ToString>(
        &mut self,
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

    fn get_fields_query<T: DeserializeOwned>() -> (String, String) {
        let fields = crate::utils::serde_introspect::<T>();
        ("fields".into(), fields.join(","))
    }

    pub async fn get_folders<T: MediaflowFolder + DeserializeOwned>(
        &mut self,
    ) -> crate::Result<Vec<T>> {
        let query = vec![Self::get_fields_query::<T>()];
        let resp = self.get_raw("folders", Some(query)).await?;
        let folders: Vec<T> = serde_json::from_str(&resp)?;
        Ok(folders)
    }

    pub async fn get_folder_children<T: MediaflowFolder + DeserializeOwned>(
        &mut self,
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
        &mut self,
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
        &mut self,
        folder_id: u32,
    ) -> crate::Result<Vec<T>> {
        let mut files: Vec<T> = self.get_folder_files(folder_id).await?;
        let subfolders: Vec<FolderId> = self.get_folder_children(folder_id).await?;
        for subfolder in subfolders {
            files.extend(self.get_folder_files_recursive(subfolder.id).await?);
        }
        Ok(files)
    }

    async fn access_token(&mut self) -> crate::Result<String> {
        if let Some(token) = self.token.take() {
            if token.close_to_expiring() {
                self.token = Some(self.authenticate().await?);
            }
            Ok(token.access_token())
        } else {
            let token = self.authenticate().await?;
            self.token = Some(token.clone());
            Ok(token.access_token())
        }
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
