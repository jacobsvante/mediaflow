use serde::{Deserialize, Serialize};

/// Represents a `/1/folder/[FOLDERID]` entity
///
/// Fields fetched from the API can be overridden using this trait. The most convenient
/// way to do so is using the derive macro:
///
///     use mediaflow::MediaflowFolder;
///     #[derive(MediaflowFolder)]
///     struct MiniFolder {
///         pub id: u32,
///         pub name: String,
///     }
pub trait MediaflowFolder {}

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
