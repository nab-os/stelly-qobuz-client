//! What crosses between the gateway and its clients, besides Qobuz's JSON.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The header a request names its lane in; absent, it is interactive.
pub const PRIORITY_HEADER: &str = "x-priority";

/// Which lane a request waits in when the account's budget is spent.
/// Interactive requests, someone at a screen, go ahead of bulk ones, a
/// crawl, an analysis, an index.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    #[default]
    Interactive,
    Bulk,
}

impl Priority {
    pub fn as_str(self) -> &'static str {
        match self {
            Priority::Interactive => "interactive",
            Priority::Bulk => "bulk",
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, String> {
        match text {
            "interactive" => Ok(Priority::Interactive),
            "bulk" => Ok(Priority::Bulk),
            _ => Err(format!("{text:?} is not a priority: interactive or bulk")),
        }
    }
}

/// Up to the asked-for number of a listing's items, and how many Qobuz says
/// there are: more than `items.len()` means the listing was cut short.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub items: Vec<Value>,
    pub total: usize,
}

/// Where a track's audio is, and what Qobuz will send there, which is not
/// always the format asked for: a track with no lossless master comes as
/// MP3, and an account that may not stream a track gets a preview.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileUrl {
    pub url: String,
    pub format_id: u32,
    pub mime_type: Option<String>,
    /// A 30s preview rather than the track.
    pub sample: bool,
    /// Seconds, of what the URL serves.
    pub duration: Option<f64>,
}

/// The body of `POST /v1/playlists`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewPlaylist {
    pub name: String,
    pub track_ids: Vec<String>,
}

/// What `POST /v1/playlists` answers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Created {
    pub id: i64,
}

/// What the gateway answers with when it cannot do what was asked.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub error: String,
}

/// The gateway said no: 404 for a track or album Qobuz does not have or
/// will not stream, 400 for a bad request, 401 for a wrong token, 502 when
/// Qobuz failed, 503 when the gateway has no credentials yet.
#[derive(Debug, Clone)]
pub struct GatewayError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for GatewayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (gateway {})", self.message, self.status)
    }
}

impl std::error::Error for GatewayError {}
