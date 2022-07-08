use mediaflow_derive::{MediaflowFile, MediaflowFolder};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone)]
pub(super) struct BearerToken {
    access_token: String,
    time_alive: Instant,
}

impl BearerToken {
    const TOKEN_MAX_AGE: u64 = 7200;

    pub(super) fn new(access_token: String) -> Self {
        Self {
            access_token,
            time_alive: Instant::now(),
        }
    }

    pub(super) fn close_to_expiring(&self) -> bool {
        (self.time_alive.elapsed().as_secs() + Self::TOKEN_MAX_AGE / 10) >= Self::TOKEN_MAX_AGE
    }

    pub(super) fn access_token(&self) -> String {
        self.access_token.clone()
    }
}

impl From<TokenResponse> for BearerToken {
    fn from(tr: TokenResponse) -> Self {
        Self::new(tr.access_token)
    }
}

#[derive(Debug, Clone, Deserialize)]
enum TokenType {
    Bearer,
}

#[derive(Debug, Clone, Deserialize)]
pub(super) struct TokenResponse {
    // NOTE: There's really no point to using refresh token currently. Perhaps in the
    //       future if we implement an authorization grant workflow.
    // refresh_token: Option<String>,
    access_token: String,
    // expires_in: u16,
    // token_type: TokenType,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, MediaflowFolder)]
#[serde(rename_all = "camelCase")]
pub struct FolderFull {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: u32,
    pub depth: u16,
    pub parent: Option<u32>,
    pub created: String,
    pub created_by: u32,
    // pub index_words: Vec<...>,
    pub has_children: bool,
    pub archived: bool,
    pub view_layout: String,
    pub separator: bool,
    pub parents: Option<Vec<ParentFolder>>,
    // pub custom_fields: ...,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, MediaflowFolder)]
#[serde(rename_all = "camelCase")]
pub struct FolderId {
    pub id: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, MediaflowFolder)]
#[serde(rename_all = "camelCase")]
pub struct ParentFolder {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: u32,
    pub depth: u16,
    pub parent: Option<u32>,
    pub has_children: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, MediaflowFile)]
#[serde(rename_all = "camelCase")]
pub struct FileBase {
    pub id: u32,
    pub name: String,
    pub filename: String,
    pub filesize: u32,
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize, MediaflowFile)]
#[serde(rename_all = "camelCase")]
pub struct FileId {
    pub id: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, MediaflowFile)]
#[serde(rename_all = "camelCase")]
pub struct FileFull {
    pub id: u32,
    pub name: String,
    pub filename: String,
    #[serde(rename = "type")]
    pub type_: FileType,
    pub filesize: u32,
    pub checksum: String,
    pub width: u16,
    pub height: u16,
    pub length: Option<u16>,
    pub dpi: u16,
    pub rating: u16,
    pub nr_downloads: u32,
    pub mark: u32,
    pub uploaded_by: u32,
    pub uploaded: String,
    // pub photo_date: ...,
    pub added_to_folder: String,
    // pub media_id: ...,
    // pub poster: ...,
    pub photographer: String,
    pub description: String,
    // pub alttext: String,
    pub instructions: String,
    pub iccprofile: Option<u16>,
    pub geodata: Option<Geodata>,
    pub processed: bool,
    pub has_preview: bool,
    pub has_versions: bool,
    pub has_history: bool,
    pub locked: bool,
    // pub focus: Focus,
    // pub colorspace: String,
    // pub bitdepth: u16,
    // pub alpha: bool,
    pub icon: Option<String>,
    pub previews: Vec<Preview>,
    pub small_preview: String,
    pub medium_preview: String,
    pub thumb_preview: String,
    pub comment: String,
    // pub usage_count: usize,
    // pub gdpr_status: String,
    // pub gdpr_type: String,
    // pub workflow_states: ...,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileType {
    pub id: u32,
    #[serde(rename = "type")]
    pub type_: String,
    pub description: String,
    pub extension: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Focus {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    pub name: String,
    pub size: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geodata {
    pub longitude: f32,
    pub latitude: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatFull {
    /// (Using signed integer because of a "custom" format with id -1)
    id: i32,
    name: String,
    width: u16,
    height: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDownloadFull {
    /// `id` here is the Format id
    /// (Using signed integer because of a "custom" format with id -1)
    id: i32,
    format: String,
    watermark: bool,
    is_preview: bool,
    filetype: String,
    expires_in: Option<u32>,
    #[serde(rename = "downloadURL")]
    download_url: String,
    download_warning: bool,
}
