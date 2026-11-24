use std::{path::PathBuf, time::Duration};

use lofty::{
    file::{AudioFile, TaggedFileExt},
    read_from_path,
    tag::Accessor,
};
pub struct Song {
    pub name: String,
    pub author: Option<String>,
    pub album: Option<String>,
    pub duration: Duration,
    pub path: PathBuf,
}

impl Song {
    pub fn from_path(path: PathBuf) -> Self {
        let file = read_from_path(&path).unwrap();
        let duration = file.properties().duration();

        let mut name_ = path.file_stem().unwrap().to_string_lossy().into_owned();
        let mut author_: Option<String> = None;
        let mut album_: Option<String> = None;
        if let Some(tag) = file.primary_tag() {
            if let Some(name) = tag.title() {
                name_ = String::from(name);
                author_ = tag.artist().map(|s| s.into_owned());
                album_ = tag.album().map(|s| s.into_owned());
            }
        }

        Self {
            name: name_,
            author: author_,
            album: album_,
            duration: duration,
            path: path,
        }
    }
}
