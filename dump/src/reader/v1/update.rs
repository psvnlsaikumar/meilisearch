use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;

use super::settings::SettingsUpdate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    data: UpdateData,
    #[serde(with = "time::serde::rfc3339")]
    enqueued_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateData {
    ClearAll,
    Customs(Vec<u8>),
    // (primary key, documents)
    DocumentsAddition {
        primary_key: Option<String>,
        documents: Vec<serde_json::Map<String, Value>>,
    },
    DocumentsPartial {
        primary_key: Option<String>,
        documents: Vec<serde_json::Map<String, Value>>,
    },
    DocumentsDeletion(Vec<String>),
    Settings(Box<SettingsUpdate>),
}

impl UpdateData {
    pub fn update_type(&self) -> UpdateType {
        match self {
            UpdateData::ClearAll => UpdateType::ClearAll,
            UpdateData::Customs(_) => UpdateType::Customs,
            UpdateData::DocumentsAddition { documents, .. } => UpdateType::DocumentsAddition {
                number: documents.len(),
            },
            UpdateData::DocumentsPartial { documents, .. } => UpdateType::DocumentsPartial {
                number: documents.len(),
            },
            UpdateData::DocumentsDeletion(deletion) => UpdateType::DocumentsDeletion {
                number: deletion.len(),
            },
            UpdateData::Settings(update) => UpdateType::Settings {
                settings: update.clone(),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum UpdateType {
    ClearAll,
    Customs,
    DocumentsAddition { number: usize },
    DocumentsPartial { number: usize },
    DocumentsDeletion { number: usize },
    Settings { settings: Box<SettingsUpdate> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessedUpdateResult {
    pub update_id: u64,
    #[serde(rename = "type")]
    pub update_type: UpdateType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_link: Option<String>,
    pub duration: f64, // in seconds
    #[serde(with = "time::serde::rfc3339")]
    pub enqueued_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub processed_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnqueuedUpdateResult {
    pub update_id: u64,
    #[serde(rename = "type")]
    pub update_type: UpdateType,
    #[serde(with = "time::serde::rfc3339")]
    pub enqueued_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum UpdateStatus {
    Enqueued {
        #[serde(flatten)]
        content: EnqueuedUpdateResult,
    },
    Failed {
        #[serde(flatten)]
        content: ProcessedUpdateResult,
    },
    Processed {
        #[serde(flatten)]
        content: ProcessedUpdateResult,
    },
}

impl UpdateStatus {
    pub fn enqueued_at(&self) -> &OffsetDateTime {
        match self {
            UpdateStatus::Enqueued { content } => &content.enqueued_at,
            UpdateStatus::Failed { content } | UpdateStatus::Processed { content } => {
                &content.enqueued_at
            }
        }
    }
}
