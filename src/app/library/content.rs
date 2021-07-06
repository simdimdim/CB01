use crate::{Message, };
use iced::{image::Handle, Element, Image};
use reqwest::Url;
use std::path::PathBuf;
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(Debug, Clone, Hash)]
pub enum Content {
    Image {
        pb:  PathBuf,
        src: Option<Url>,
    },
    Text {
        pb:   PathBuf,
        src:  Option<Url>,
        text: String,
    },
    Other {
        pb:  PathBuf,
        src: Option<Url>,
    },
    Empty,
}
impl Content {
    pub fn visual(&self) -> bool {
        match self {
            Self::Image { .. } => true,
            Self::Text { .. } => false,
            Self::Other { .. } => false,
            Self::Empty => false,
        }
    }

    pub async fn save(&self, mut pb: PathBuf) {
        std::fs::create_dir_all(&pb).unwrap();
        if self.visual() {
            pb.set_extension("jpg");
        }
        match self {
            Self::Image { .. } => {
                let _f = Handle::from_path(&pb);
            }
            Self::Text { pb, text, .. } => {
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(pb)
                    .await
                    .expect("Missing content file.")
                    .write(text.as_bytes())
                    .await
                    .unwrap();
            }
            Self::Other { .. } => (),
            Self::Empty => (),
        }
    }

    pub async fn load(&mut self, path: PathBuf) -> Self {
        if path.extension() == Some("jpg".as_ref()) {
            Content::Image {
                pb:  path,
                src: None,
            }
        } else {
            let mut buf = vec![];
            OpenOptions::new()
                .read(true)
                .open(&path)
                .await
                .expect("Missing content file.")
                .read_to_end(&mut buf)
                .await
                .unwrap();
            Content::Text {
                pb:   path,
                src:  None,
                text: String::from_utf8(buf).unwrap(),
            }
        }
    }

    // tempt, to be adopted by the translator struct/macro
    pub fn view(&self, columns: Option<u16>) -> Element<Message> {
        use iced::{HorizontalAlignment, Length, Text, VerticalAlignment};
        match self {
            Self::Image { pb: path, .. } => Image::new(Handle::from_path(path))
                .width(Length::FillPortion(columns.unwrap_or(1)))
                .height(Length::FillPortion(columns.unwrap_or(1)))
                .into(),
            Self::Text { text, .. } => Text::new(text.clone())
                .width(Length::Fill)
                .vertical_alignment(VerticalAlignment::Top)
                .horizontal_alignment(HorizontalAlignment::Center)
                .into(),
            Self::Other { .. } => Text::new("Unable to preview.")
                .width(Length::Fill)
                .vertical_alignment(VerticalAlignment::Top)
                .horizontal_alignment(HorizontalAlignment::Center)
                .into(),
            Self::Empty => Text::new("There's no content here.")
                .width(Length::Fill)
                .vertical_alignment(VerticalAlignment::Top)
                .horizontal_alignment(HorizontalAlignment::Center)
                .into(),
        }
    }
}
impl Default for Content {
    fn default() -> Self { Self::Empty }
}
impl From<Vec<String>> for Content {
    fn from(text: Vec<String>) -> Self {
        Self::Text {
            pb:   PathBuf::from("library/unsorted"),
            text: text.join("\n\n"),
            src:  None,
        }
    }
}
impl Eq for Content {}
impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Image { pb, .. }, Self::Image { pb: p, .. }) => pb == p,
            (Self::Text { pb, text, .. }, Self::Text { pb: p, text: t, .. }) => {
                pb == p && text == t
            }
            (Self::Empty, Self::Empty) => true,
            (Self::Other { pb, .. }, Self::Other { pb: p2, .. }) => pb == p2,
            (Content::Image { .. }, Content::Text { .. }) => false,
            (Content::Image { .. }, Content::Other { .. }) => false,
            (Content::Image { .. }, Content::Empty) => false,
            (Content::Text { .. }, Content::Image { .. }) => false,
            (Content::Text { .. }, Content::Other { .. }) => false,
            (Content::Text { .. }, Content::Empty) => false,
            (Content::Other { .. }, Content::Image { .. }) => false,
            (Content::Other { .. }, Content::Text { .. }) => false,
            (Content::Other { .. }, Content::Empty) => false,
            (Content::Empty, Content::Image { .. }) => false,
            (Content::Empty, Content::Text { .. }) => false,
            (Content::Empty, Content::Other { .. }) => false,
        }
    }
}
