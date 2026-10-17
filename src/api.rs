//! Every call the gateway answers, named once: the gateway routes these
//! paths, and either client sends what these build.

use crate::wire::{Created, FileUrl, NewPlaylist, Page};
use serde_json::Value;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
}

/// One request to the gateway, and what it answers with.
#[derive(Debug, Clone)]
pub struct Call<T> {
    pub method: Method,
    /// From the gateway's root, `/v1/...`; ids are percent-encoded.
    pub path: String,
    pub query: Vec<(&'static str, String)>,
    pub body: Option<Value>,
    answer: PhantomData<fn() -> T>,
}

impl<T> Call<T> {
    fn get(path: String) -> Self {
        Self {
            method: Method::Get,
            path,
            query: Vec::new(),
            body: None,
            answer: PhantomData,
        }
    }

    fn query(mut self, key: &'static str, value: impl ToString) -> Self {
        self.query.push((key, value.to_string()));
        self
    }
}

/// Ids go into paths: anything but what Qobuz ids are made of is encoded.
fn id(text: &str) -> String {
    text.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' => (b as char).to_string(),
            _ => format!("%{b:02X}"),
        })
        .collect()
}

/// Whether the gateway is up, credentials or not.
pub fn health() -> Call<Value> {
    Call::get("/v1/health".into())
}

/// The account, as `user/get` has it.
pub fn user() -> Call<Value> {
    Call::get("/v1/user".into())
}

/// `catalog/search`: tracks, albums and artists, or only `kind` of them.
pub fn search(query: &str, limit: usize, kind: Option<&str>) -> Call<Value> {
    let call = Call::get("/v1/search".into()).query("q", query).query("limit", limit);
    match kind {
        Some(kind) => call.query("type", kind),
        None => call,
    }
}

/// `track/get`.
pub fn track(track_id: &str) -> Call<Value> {
    Call::get(format!("/v1/tracks/{}", id(track_id)))
}

/// A signed URL for a track's audio in `format_id` (5 MP3 320, 6 FLAC 16/44.1).
pub fn file_url(track_id: &str, format_id: u32) -> Call<FileUrl> {
    Call::get(format!("/v1/tracks/{}/file", id(track_id))).query("format", format_id)
}

/// `album/get`, with the whole tracklist.
pub fn album(album_id: &str) -> Call<Value> {
    Call::get(format!("/v1/albums/{}", id(album_id)))
}

/// `artist/get`.
pub fn artist(artist_id: &str) -> Call<Value> {
    Call::get(format!("/v1/artists/{}", id(artist_id)))
}

/// An artist's albums through `artist/get`, with no release type.
pub fn artist_albums(artist_id: &str, cap: usize) -> Call<Page> {
    Call::get(format!("/v1/artists/{}/albums", id(artist_id))).query("cap", cap)
}

/// An artist's releases through `artist/getReleasesList`, newest first.
/// `release_type` is `all` or a comma list of `album`, `epSingle`, `live`,
/// `other`.
pub fn artist_releases(artist_id: &str, release_type: &str, cap: usize) -> Call<Page> {
    Call::get(format!("/v1/artists/{}/releases", id(artist_id)))
        .query("type", release_type)
        .query("cap", cap)
}

/// Artists Qobuz considers close to this one.
pub fn similar_artists(artist_id: &str, limit: usize) -> Call<Vec<Value>> {
    Call::get(format!("/v1/artists/{}/similar", id(artist_id))).query("limit", limit)
}

/// The account's favourite `kind`: tracks, albums or artists.
pub fn favourites(kind: &str, cap: usize) -> Call<Page> {
    Call::get(format!("/v1/favourites/{}", id(kind))).query("cap", cap)
}

/// The account's playlists.
pub fn playlists(cap: usize) -> Call<Page> {
    Call::get("/v1/playlists".into()).query("cap", cap)
}

/// A playlist's tracks.
pub fn playlist_tracks(playlist_id: &str, cap: usize) -> Call<Page> {
    Call::get(format!("/v1/playlists/{}/tracks", id(playlist_id))).query("cap", cap)
}

/// A new private playlist of `track_ids`, in order.
pub fn create_playlist(name: &str, track_ids: &[String]) -> Call<Created> {
    let body = NewPlaylist {
        name: name.to_string(),
        track_ids: track_ids.to_vec(),
    };
    Call {
        method: Method::Post,
        path: "/v1/playlists".into(),
        query: Vec::new(),
        body: Some(serde_json::to_value(body).expect("a playlist serialises")),
        answer: PhantomData,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_cannot_leave_their_path_segment() {
        assert_eq!(track("123").path, "/v1/tracks/123");
        assert_eq!(album("0060254728690").path, "/v1/albums/0060254728690");
        assert_eq!(album("../user").path, "/v1/albums/..%2Fuser");
        assert_eq!(search("a b", 5, None).query, [("q", "a b".to_string()), ("limit", "5".to_string())]);
    }
}
