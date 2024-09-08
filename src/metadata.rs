use dbus::arg::PropMap;

use crate::utils;

#[derive(Default, Debug)]
pub struct Metadata {
    pub trackid: Option<String>,
    pub length: Option<i64>,
    pub art_url: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<Vec<String>>,
    pub artist: Option<Vec<String>>,
    pub as_text: Option<String>,
    pub audio_bpm: Option<i64>,
    pub auto_rating: Option<f64>,
    pub comment: Option<Vec<String>>,
    pub composer: Option<Vec<String>>,
    pub content_created: Option<String>,
    pub disc_number: Option<i64>,
    pub first_used: Option<String>,
    pub genre: Option<Vec<String>>,
    pub last_used: Option<String>,
    pub lyricist: Option<Vec<String>>,
    pub title: Option<String>,
    pub track_number: Option<i64>,
    pub url: Option<String>,
    pub use_count: Option<i64>,
    pub user_rating: Option<f64>,
}

impl<'a> Metadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(metadata_str: &PropMap) -> Result<Self, serde_json::Error> {
        let mut metadata = Self::new();

        for (key, value) in metadata_str {
            match key.as_str() {
                "mpris:trackid" => {
                    metadata.trackid = utils::get_string(value).unwrap_or_default().into()
                }
                "mpris:length" => metadata.length = Some(utils::get_i64(value).unwrap_or_default()),
                "mpris:artUrl" => {
                    metadata.art_url = Some(utils::get_string(value).unwrap_or_default())
                }
                "xesam:album" => {
                    metadata.album = Some(utils::get_string(value).unwrap_or_default())
                }
                "xesam:albumArtist" => {
                    metadata.album_artist = Some(utils::get_string_vec(value).unwrap_or_default())
                }
                "xesam:artist" => {
                    metadata.artist = Some(utils::get_string_vec(value).unwrap_or_default())
                }
                "xesam:asText" => {
                    metadata.as_text = Some(utils::get_string(value).unwrap_or_default())
                }
                "xesam:audioBPM" => {
                    metadata.audio_bpm = Some(utils::get_i64(value).unwrap_or_default())
                }
                "xesam:autoRating" => {
                    metadata.auto_rating = Some(utils::get_f64(value).unwrap_or_default())
                }
                "xesam:comment" => {
                    metadata.comment = Some(utils::get_string_vec(value).unwrap_or_default())
                }
                "xesam:composer" => {
                    metadata.composer = Some(utils::get_string_vec(value).unwrap_or_default())
                }
                "xesam:contentCreated" => {
                    metadata.content_created = Some(utils::get_string(value).unwrap_or_default())
                }
                "xesam:discNumber" => {
                    metadata.disc_number = Some(utils::get_i64(value).unwrap_or_default())
                }
                "xesam:firstUsed" => {
                    metadata.first_used = Some(utils::get_string(value).unwrap_or_default())
                }
                "xesam:genre" => {
                    metadata.genre = Some(utils::get_string_vec(value).unwrap_or_default())
                }
                "xesam:lastUsed" => {
                    metadata.last_used = Some(utils::get_string(value).unwrap_or_default())
                }
                "xesam:lyricist" => {
                    metadata.lyricist = Some(utils::get_string_vec(value).unwrap_or_default())
                }
                "xesam:title" => {
                    metadata.title = Some(utils::get_string(value).unwrap_or_default())
                }
                "xesam:trackNumber" => {
                    metadata.track_number = Some(utils::get_i64(value).unwrap_or_default())
                }
                "xesam:url" => metadata.url = Some(utils::get_string(value).unwrap_or_default()),
                "xesam:useCount" => {
                    metadata.use_count = Some(utils::get_i64(value).unwrap_or_default())
                }
                "xesam:userRating" => {
                    metadata.user_rating = Some(utils::get_f64(value).unwrap_or_default())
                }
                _ => {}
            }
        }

        Ok(metadata)
    }
}
