use serde::{Deserialize, Serialize};

pub trait MediaflowFile {}
pub trait MediaflowFolder {}
pub trait MediaflowFileDownload {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
impl MediaflowFolder for FolderFull {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderId {
    pub id: u32,
}
impl MediaflowFolder for FolderId {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
impl MediaflowFolder for ParentFolder {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileBase {
    pub id: u32,
    pub name: String,
    pub filename: String,
    pub filesize: u32,
}
impl MediaflowFile for FileBase {}

#[derive(Default, Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileId {
    pub id: u32,
}
impl MediaflowFile for FileId {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
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
impl MediaflowFile for FileFull {}

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
