//! stelly-qobuz's gateway from the other side. The gateway holds the Qobuz
//! account, its credentials and its one request queue; whatever wants
//! Qobuz, Stelly's server, stelly-mix, asks it over HTTP instead.
//!
//! ```ignore
//! let gateway = stelly_qobuz_client::Client::new(Config::from_env(None));
//! let album: serde_json::Value = gateway.send(api::album("0060254728690")).await?;
//! let url = gateway.at(Priority::Bulk).send(api::file_url("12345", 5)).await?;
//! ```
//!
//! Reads answer with Qobuz's own JSON. `api` names every call once, and both
//! clients send any of them; the `async` and `blocking` features add them.

pub mod api;
pub mod config;
pub mod wire;

#[cfg(feature = "async")]
mod async_client;
#[cfg(feature = "blocking")]
pub mod blocking;

#[cfg(feature = "async")]
pub use async_client::Client;
pub use config::Config;
pub use wire::{FileUrl, GatewayError, Page, Priority};
