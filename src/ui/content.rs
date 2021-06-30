use std::path::PathBuf;
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt},
};

pub type Manga = Box<Vec<u8>>;
pub type Novel = String;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Content {
    Image(Manga),
    Text(Novel),
}
impl Content {
    pub fn visual(&self) -> bool {
        match self {
            Content::Image(_) => true,
            Content::Text(_) => false,
        }
    }

    pub async fn save(&self, mut pb: PathBuf) {
        std::fs::create_dir_all(&pb).unwrap();
        if self.visual() {
            pb.set_extension("jpg");
        }
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(pb)
            .await
            .unwrap()
            .write(self.data())
            .await
            .unwrap();
    }

    pub async fn load(&mut self, pb: PathBuf) -> Self {
        let mut buf = vec![];
        let mut f = OpenOptions::new()
            .read(true)
            .open(&pb)
            .await
            .map_or(
                {
                    let mut pb = pb.clone();
                    pb.set_extension("jpg");
                    OpenOptions::new().read(true).open(pb).await
                },
                |f| Ok(f),
            )
            .expect("Missing content file.");
        f.read_to_end(&mut buf).await.unwrap();
        if pb.extension() == Some("jpg".as_ref()) {
            Content::Image(buf.into())
        } else {
            Content::Image(buf.into()).to_novel().unwrap_or_else(|a| a)
        }
    }

    pub fn to_novel(self) -> Result<Self, Self> {
        if let Self::Image(m) = self.clone() {
            String::from_utf8(*m).map(Into::into).or(Err(self))
        } else {
            Err(self)
        }
    }

    fn data(&self) -> &[u8] {
        match self {
            Content::Image(m) => &m,
            Content::Text(n) => n.as_bytes(),
        }
    }
}

impl From<Novel> for Content {
    fn from(text: Novel) -> Self { Self::Text(text) }
}
impl From<Manga> for Content {
    fn from(image: Manga) -> Self { Self::Image(image) }
}
impl From<Vec<String>> for Content {
    fn from(text: Vec<String>) -> Self { Self::Text(text.join("\n\n")) }
}
impl From<Vec<u8>> for Content {
    fn from(image: Vec<u8>) -> Self { Self::Image(Box::new(image)) }
}
