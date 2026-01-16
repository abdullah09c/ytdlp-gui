use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaylistVideo {
    pub index: usize,
    pub title: String,
    pub video_id: String,
    pub duration: Option<u64>,
}

impl core::fmt::Display for PlaylistVideo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let duration_str = if let Some(duration) = self.duration {
            let mins = duration / 60;
            let secs = duration % 60;
            format!(" ({:02}:{:02})", mins, secs)
        } else {
            String::new()
        };
        write!(f, "{}. {}{}", self.index, self.title, duration_str)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct YtDlpPlaylistInfo {
    pub id: Option<String>,
    pub title: Option<String>,
    pub duration: Option<u64>,
}
