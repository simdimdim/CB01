use crate::{Book, Id, Message};
use iced::{
    alignment::{Horizontal, Vertical},
    image::Handle,
    Color,
    Element,
    Image,
    Length,
    Text,
};
use itertools::Itertools;
use log::{error, info};
use serde::{Deserialize as de, Serialize as se};
use std::{
    collections::{BTreeMap, BTreeSet},
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, se, de)]
pub enum ContentType {
    Image,
    Text,
    Other,
    #[default]
    Empty,
}
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, se, de)]
pub struct File {
    pub name:     Option<OsString>,
    pub ext:      Option<OsString>,
    pub path:     PathBuf,
    pub filetype: ContentType,
}
#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash, se, de)]
pub struct Folder {
    pub path:     PathBuf,
    pub folders:  BTreeSet<Folder>,
    pub files:    BTreeSet<File>,
    pub explored: bool,
    pub recurse:  bool,
}

impl File {
    pub fn name(&self) -> Option<&OsStr> { self.path.file_name() }

    pub fn text(&self) -> String { String::new() }

    pub fn view(&self, cols: Option<u16>, dark: bool) -> Element<'_, Message> {
        match self.filetype {
            ContentType::Image => {
                Image::new(Handle::from_path(self.path.clone()))
                    .width(Length::FillPortion(cols.unwrap_or(1)))
                    .height(Length::Fill)
                    .into()
            }
            ContentType::Text => Text::new(self.text())
                .width(Length::Fill)
                .vertical_alignment(Vertical::Top)
                .horizontal_alignment(Horizontal::Center)
                .into(),
            ContentType::Other => Text::new("Unable to preview.")
                .width(Length::Fill)
                .vertical_alignment(Vertical::Top)
                .horizontal_alignment(Horizontal::Center)
                .into(),
            ContentType::Empty => Text::new("There's no content here.")
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
impl Folder {
    pub fn name(&self) -> Option<&OsStr> { self.path.file_name() }

    pub fn add_file(&mut self, path: PathBuf) { self.files.insert(path.into()); }

    pub fn explore(&mut self, levels: Option<u8>) {
        let n = levels.unwrap_or(3);
        fn explr(folder: &mut Folder, path: PathBuf, level: u8) {
            if level < 1 {
                return;
            }
            if path.is_dir() {
                let mut sub: Folder = path.into();
                sub.recurse = folder.recurse;
                folder.folders.insert(sub);
            } else if path.is_file() {
                folder.files.insert(path.into());
            } else if path.is_symlink() {
                if let Ok(link) = path.read_link() {
                    explr(folder, link, level - 1)
                }
            }
        }
        for path in self.path.read_dir().expect("Failed to read dir.").flatten() {
            let path = path.path().to_path_buf();
            explr(self, path, n - 1);
        }
        self.explored = true;
    }

    pub fn top_level(self) -> Vec<File> {
        self.files
            .into_iter()
            .sorted_by(|f1, f2| f1.cmp(f2))
            .collect()
    }

    pub async fn all_files(&self) -> BTreeMap<usize, Vec<File>> {
        let mut file_list = BTreeMap::new();
        file_list.insert(0, self.clone().top_level());
        file_list.append((1usize..).zip(self.folders.clone().into_iter()).fold(
            &mut BTreeMap::new(),
            |acc, (n, f)| {
                if f.explored {
                    acc.insert(n, f.top_level());
                }
                acc
            },
        ));
        file_list
    }
}

pub async fn explore(path: PathBuf, depth: u8) -> Folder {
    let mut folder = Folder::default();
    if path.is_dir() {
        if let Ok(dir) = path.read_dir() {
            for de in dir.filter_map(Result::ok) {
                if let Ok(f) = de.file_type() {
                    let path = de.path().to_path_buf();
                    if f.is_dir() {
                        folder.folders.insert(path.into());
                    } else if f.is_file() {
                        folder.files.insert(path.into());
                    } else if f.is_symlink() {
                        if let Ok(sym) = path.read_link() {
                            if sym.is_dir() {
                                folder.folders.insert(sym.into());
                            } else if sym.is_file() {
                                folder.files.insert(sym.into());
                            }
                        }
                    }
                }
            }
        } else {
            info!("Could not read dir.")
        };
    } else if path.is_file() {
        folder.files.insert(path.clone().into());
        if let Some(p) = path.parent() {
            folder.path = p.to_path_buf();
        }
    } else if path.is_symlink() {
        match std::fs::read_link(&folder.path) {
            Ok(_p) => {
                if depth > 1 {
                    // folder.folders.push(explore(p, depth - 1).await);
                }
            }
            Err(_) => error!("Couldn't read dir."),
        };
    }
    folder
}

impl From<PathBuf> for File {
    fn from(path: PathBuf) -> Self {
        File {
            name: path.file_name().map(|name| name.to_os_string()),
            ext: path.extension().map(|ext| ext.to_os_string()),
            filetype: contenttype(path.as_path()),
            path,
        }
    }
}
impl From<PathBuf> for Folder {
    fn from(path: PathBuf) -> Self {
        Folder {
            path,
            ..Default::default()
        }
    }
}

fn contenttype(path: &Path) -> ContentType {
    let pictures: Vec<&OsStr> = vec![
        OsStr::new("jpg"),
        OsStr::new("jpeg"),
        OsStr::new("bmp"),
        OsStr::new("png"),
    ];
    let text = vec![OsStr::new("txt")];
    if let Some(ext) = path.extension() {
        match ext {
            _ if pictures.contains(&ext) => ContentType::Image,
            _ if text.contains(&ext) => ContentType::Text,
            _ => ContentType::Other,
        }
    } else {
        ContentType::default()
    }
}

pub fn save_book(book: Book, loc: Option<PathBuf>) {
    let _loc = match loc {
        Some(loc) => loc,
        None => book.folder.path,
    };
}
pub fn load_books(_loc: PathBuf) -> Vec<Book> { vec![] }
