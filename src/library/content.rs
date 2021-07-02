use iced::image::Handle;
use std::path::PathBuf;
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(Debug, Clone)]
pub enum Content {
    Image(Handle),
    Text(String),
    Empty,
}
impl Content {
    pub fn visual(&self) -> bool {
        match self {
            Self::Image(_) => true,
            Self::Text(_) => false,
            Self::Empty => false,
        }
    }

    pub async fn save(&self, mut pb: PathBuf) {
        std::fs::create_dir_all(&pb).unwrap();
        if self.visual() {
            pb.set_extension("jpg");
        }
        match self {
            Self::Image(_h) => {
                let _f = Handle::from_path(&pb);
            }
            Self::Text(s) => {
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&pb)
                    .await
                    .expect("Missing content file.")
                    .write(s.as_bytes())
                    .await
                    .unwrap();
            }
            Self::Empty => (),
        }
    }

    pub async fn load(&mut self, pb: PathBuf) -> Self {
        if pb.extension() == Some("jpg".as_ref()) {
            let handle = Handle::from_path(&pb);
            Content::Image(handle)
        } else {
            let mut buf = vec![];
            OpenOptions::new()
                .read(true)
                .open(&pb)
                .await
                .expect("Missing content file.")
                .read_to_end(&mut buf)
                .await
                .unwrap();
            Content::Text(String::from_utf8(buf).unwrap())
        }
    }
}
impl Default for Content {
    fn default() -> Self { Self::Empty }
}
impl From<Vec<String>> for Content {
    fn from(text: Vec<String>) -> Self { Self::Text(text.join("\n\n")) }
}
impl From<Vec<u8>> for Content {
    fn from(text: Vec<u8>) -> Self { Self::Image(Handle::from_memory(text)) }
}
