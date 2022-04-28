use std::time::Instant;
use mediaflow_derive::{MediaflowFile, MediaflowFolder};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone)]
pub(super) struct BearerToken {
    access_token: String,
    time_alive: Instant,
}

impl BearerToken {
    const TOKEN_MAX_AGE: u64 = 7200;

    pub(super) fn new(
        access_token: String,
    ) -> Self {
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
    name: String,
    #[serde(rename = "type")]
    type_: u32,
    depth: u16,
    parent: Option<u32>,
    created: String,
    created_by: u32,
    // index_words: Vec<...>,
    has_children: bool,
    archived: bool,
    view_layout: String,
    separator: bool,
    parents: Option<Vec<ParentFolder>>,
    // custom_fields: ...,
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
    name: String,
    #[serde(rename = "type")]
    type_: u32,
    depth: u16,
    parent: Option<u32>,
    has_children: bool,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, MediaflowFile)]
#[serde(rename_all = "camelCase")]
pub struct FileBase {
    pub id: u32,
    name: String,
    filename: String,
    filesize: u32,
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
    name: String,
    filename: String,
    #[serde(rename = "type")]
    type_: FileType,
    filesize: u32,
    checksum: String,
    width: u16,
    height: u16,
    length: Option<u16>,
    dpi: u16,
    rating: u16,
    nr_downloads: u32,
    mark: u32,
    uploaded_by: u32,
    uploaded: String,
    // photo_date: ...,
    added_to_folder: String,
    // media_id: ...,
    // poster: ...,
    photographer: String,
    description: String,
    // alttext: String,
    instructions: String,
    iccprofile: Option<u16>,
    geodata: Option<Geodata>,
    processed: bool,
    has_preview: bool,
    has_versions: bool,
    has_history: bool,
    locked: bool,
    // focus: Focus,
    // colorspace: String,
    // bitdepth: u16,
    // alpha: bool,
    icon: Option<String>,
    previews: Vec<Preview>,
    small_preview: String,
    medium_preview: String,
    thumb_preview: String,
    comment: String,
    // usage_count: usize,
    // gdpr_status: String,
    // gdpr_type: String,
    // workflow_states: ...,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileType {
    pub id: u32,
    #[serde(rename = "type")]
    type_: String,
    description: String,
    extension: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Focus {
    x: f32,
    y: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    name: String,
    size: String,
    url: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geodata {
    longitude: f32,
    latitude: f32,
}
