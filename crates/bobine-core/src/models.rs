use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VolumeKind {
    #[default]
    Internal,
    External,
    Nas,
    Card,
}

impl ToSql for VolumeKind {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let value = match self {
            Self::Internal => "internal",
            Self::External => "external",
            Self::Nas => "nas",
            Self::Card => "card",
        };
        Ok(ToSqlOutput::from(value))
    }
}

impl FromSql for VolumeKind {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "internal" => Ok(Self::Internal),
            "external" => Ok(Self::External),
            "nas" => Ok(Self::Nas),
            "card" => Ok(Self::Card),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClipKind {
    #[default]
    Video,
    Photo,
}

impl ToSql for ClipKind {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let value = match self {
            Self::Video => "video",
            Self::Photo => "photo",
        };
        Ok(ToSqlOutput::from(value))
    }
}

impl FromSql for ClipKind {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "video" => Ok(Self::Video),
            "photo" => Ok(Self::Photo),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub id: String,
    pub name: String,
    pub kind: VolumeKind,
    pub stable_id: String,
    pub last_path: Option<String>,
    pub connected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Default)]
pub struct NewClip {
    pub id: String,
    pub content_hash: Option<String>,
    pub kind: ClipKind,
    pub original_filename: String,
    pub project_id: String,
    pub created_at: String,
    pub duration_ms: Option<i64>,
    pub captured_at: Option<String>,
    pub codec: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub file_size: Option<i64>,
    pub bitrate: Option<i64>,
    pub fps: Option<f64>,
    pub camera_type: Option<String>,
    pub color_space: Option<String>,
    pub poster_path: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NewLocation {
    pub id: String,
    pub clip_id: String,
    pub volume_id: String,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipSummary {
    pub id: String,
    pub kind: ClipKind,
    pub original_filename: String,
    pub project_id: String,
    pub project_name: String,
    pub duration_ms: Option<i64>,
    pub camera_type: Option<String>,
    pub color_space: Option<String>,
    pub locations: Vec<ClipLocationView>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipLocationView {
    pub volume_id: String,
    pub volume_name: String,
    pub path: String,
    pub connected: bool,
}
