use serde::{Deserialize, Serialize};

/// Represents an entity from `/1/file/[FILEID]/downloads`
///
/// Fields fetched from the API can be overridden using this trait. The most convenient
/// way to do so is using the derive macro:
///
///     use mediaflow::MediaflowFileDownload;
///     #[derive(MediaflowFileDownload)]
///     struct MiniFileDownload {
///         pub id: u32,
///         pub name: String,
///     }
pub trait MediaflowFileDownload {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDownloadFull {
    /// `id` here is the Format id
    /// (Using signed integer because of a "custom" format with id -1)
    pub id: i32,
    pub format: String,
    pub watermark: bool,
    pub is_preview: bool,
    pub filetype: String,
    pub expires_in: Option<u32>,
    #[serde(rename = "downloadURL")]
    pub download_url: String,
    pub download_warning: bool,
}
impl MediaflowFileDownload for FileDownloadFull {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDownloadUrl {
    /// `id` here is the Format id
    /// (Using signed integer because of a "custom" format with id -1)
    pub id: i32,
    #[serde(rename = "downloadURL")]
    pub download_url: String,
}
impl MediaflowFileDownload for FileDownloadUrl {}
