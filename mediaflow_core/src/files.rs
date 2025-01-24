use serde::{Deserialize, Serialize};

/// Represents a `/1/file/[FILEID]` entity
///
/// Fields fetched from the API can be overridden using this trait. The most convenient
/// way to do so is using the derive macro:
///
///     use mediaflow::MediaflowFile;
///     #[derive(MediaflowFile)]
///     struct MiniFile {
///         pub id: u32,
///         pub name: String,
///     }
pub trait MediaflowFile {}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileBase {
    pub id: u32,
    pub name: String,
    pub filename: String,
    pub filesize: u32,
}
impl MediaflowFile for FileBase {}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
    pub width: u32,
    pub height: u32,
    pub length: Option<u32>,
    pub dpi: u32,
    pub rating: u32,
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
    pub iccprofile: Option<u32>,
    pub geodata: Option<Geodata>,
    pub processed: bool,
    pub has_preview: bool,
    pub has_versions: bool,
    pub has_history: bool,
    pub locked: bool,
    // pub focus: Focus,
    // pub colorspace: String,
    // pub bitdepth: u32,
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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
