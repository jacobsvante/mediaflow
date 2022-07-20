use serde::{Deserialize, Serialize};

/// Represents a `/1/format/[FORMATID]` entity
///
/// Fields fetched from the API can be overridden using this trait. The most convenient
/// way to do so is using the derive macro:
///
///     use mediaflow::MediaflowFormat;
///     #[derive(MediaflowFormat)]
///     struct MiniFormat {
///         pub id: u32,
///         pub name: String,
///     }
pub trait MediaflowFormat {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormatFull {
    /// (Using signed integer because of a "custom" format with id -1)
    id: i32,
    name: String,
    width: u16,
    height: u16,
}

impl MediaflowFormat for FormatFull {}
