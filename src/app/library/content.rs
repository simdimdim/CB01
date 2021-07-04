use crate::Message;
use iced::{image::Handle, Element, Image};
use std::path::PathBuf;
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(Debug, Clone, Hash)]
pub enum Content {
    Image(PathBuf),
    Text(PathBuf, String),
    Other(PathBuf),
    Empty,
}
impl Content {
    pub fn visual(&self) -> bool {
        match self {
            Self::Image(_) => true,
            Self::Text(..) => false,
            Self::Other(..) => false,
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
            Self::Text(pb, s) => {
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
            Self::Other(..) => (),
            Self::Empty => (),
        }
    }

    pub async fn load(&mut self, pb: PathBuf) -> Self {
        if pb.extension() == Some("jpg".as_ref()) {
            Content::Image(pb)
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
            Content::Text(pb, String::from_utf8(buf).unwrap())
        }
    }

    // tempt, to be adopted by the translator struct/macro
    pub fn view(&self, columns: Option<u16>) -> Element<Message> {
        use iced::{HorizontalAlignment, Length, Text, VerticalAlignment};
        match self {
            Self::Image(pb) => Image::new(Handle::from_path(pb))
                .width(Length::FillPortion(columns.unwrap_or(1)))
                .height(Length::FillPortion(columns.unwrap_or(1)))
                .into(),
            Self::Text(_, t) => Text::new(t.clone())
                .width(Length::Fill)
                .vertical_alignment(VerticalAlignment::Top)
                .horizontal_alignment(HorizontalAlignment::Center)
                .into(),
            Self::Other(..) => Text::new("Unable to preview.")
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
        Self::Text(PathBuf::from("library/unsorted"), text.join("\n\n"))
    }
}
impl Eq for Content {}
impl PartialEq for Content {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Image(_), Self::Image(_)) => todo!(),
            (Self::Text(pb1, s), Self::Text(pb2, t)) => pb1 == pb2 && s == t,
            (Self::Empty, Self::Empty) => true,
            (Self::Other(pb1), Self::Other(pb2)) => pb1 == pb2,
            (Self::Image(_), Self::Text(_, _)) => false,
            (Self::Image(_), Self::Other(_)) => false,
            (Self::Image(_), Self::Empty) => false,
            (Self::Text(_, _), Self::Image(_)) => false,
            (Self::Text(_, _), Self::Other(_)) => false,
            (Self::Text(_, _), Self::Empty) => false,
            (Self::Other(_), Self::Image(_)) => false,
            (Self::Other(_), Self::Text(_, _)) => false,
            (Self::Other(_), Self::Empty) => false,
            (Self::Empty, Self::Image(_)) => false,
            (Self::Empty, Self::Text(_, _)) => false,
            (Self::Empty, Self::Other(_)) => false,
        }
    }
}
