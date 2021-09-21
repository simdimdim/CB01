use crate::{Message, Theme};
use iced::{image::Handle, Color, Element, Image};
use reqwest::Url;
use std::path::PathBuf;
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt},
};

static MISSING: &str = "Missing content file.";

#[derive(Debug, Clone, PartialOrd, Ord)]
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

    pub async fn save(&mut self, data: &'_ [u8]) {
        match self {
            Self::Image { pb, .. } => {
                pb.set_extension("jpg");
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(pb)
                    .await
                    .expect(MISSING)
                    .write_all(data)
                    .await
                    .unwrap();
                // let _f = Handle::from_path(&pb);
            }
            Self::Text { pb, text, .. } => {
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(pb)
                    .await
                    .expect(MISSING)
                    .write_all(text.as_bytes())
                    .await
                    .unwrap();
            }
            Self::Other { .. } => (),
            Self::Empty => (),
        }
    }

    pub async fn load(&mut self, pb: PathBuf) -> Self {
        if pb.extension() == Some("jpg".as_ref()) {
            Content::Image { pb, src: None }
        } else {
            let mut buf = vec![];
            OpenOptions::new()
                .read(true)
                .open(&pb)
                .await
                .expect(MISSING)
                .read_to_end(&mut buf)
                .await
                .unwrap();
            Content::Text {
                pb,
                src: None,
                text: String::from_utf8(buf).unwrap(),
            }
        }
    }

    // tempt, to be adopted by the translator struct/macro
    pub fn view(&self, cols: Option<u16>, dark: bool) -> Element<'_, Message> {
        use iced::{
            alignment::{Horizontal, Vertical},
            Length,
            Text,
        };
        //Portion(columns.unwrap_or(1))
        match self {
            Self::Image { pb, .. } => Image::new(Handle::from_path(pb))
                .width(Length::FillPortion(cols.unwrap_or(1)))
                .height(Length::Fill)
                .into(),
            Self::Text { text, .. } => Text::new(text.clone())
                .width(Length::Fill)
                .vertical_alignment(Vertical::Top)
                .horizontal_alignment(Horizontal::Center)
                .into(),
            Self::Other { .. } => Text::new("Unable to preview.")
                .width(Length::Fill)
                .vertical_alignment(Vertical::Top)
                .horizontal_alignment(Horizontal::Center)
                .into(),
            Self::Empty => Text::new("There's no content here.")
                .width(Length::Fill)
                .color({
                    let c = if dark { 255. } else { 0. };
                    Color {
                        r: c,
                        g: c,
                        b: c,
                        a: 1.,
                    }
                })
                .vertical_alignment(Vertical::Top)
                .horizontal_alignment(Horizontal::Center)
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
impl std::hash::Hash for Content {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Image { pb, .. } => pb.hash(state),
            Self::Text { pb, text, .. } => {
                pb.hash(state);
                text.hash(state);
            }
            Self::Other { pb, .. } => pb.hash(state),
            Self::Empty => (),
        }
    }
}
