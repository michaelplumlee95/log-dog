use chrono::{DateTime, Utc};
use serde::Deserialize;

/// One normalized log event
#[derive(Debug, Clone, Deserialize)]
pub struct Event {
    /// Timestamp
    pub ts: DateTime<Utc>,

    /// Subsystem or Component
    pub system: String,

    /// Sev level, can be enum later
    pub level: String,

    /// Human readable message
    pub msg: String,

    /// Host/component identifier (optional)
    #[serde(default)]
    pub host: Option<String>,

    /// Error/Alarm code (optional)
    #[serde(default)]
    pub code: Option<String>,
}
