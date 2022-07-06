use crate::{Message, APP_NAME, BLACK, WHITE};
use directories::ProjectDirs;
use iced::{checkbox, Background, Color};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Copy, Clone)]
pub enum AppState {
    Settings,
    Reader,
    Library,
    Info,
    Add,
}
impl Default for AppState {
    fn default() -> Self { Self::Library }
}
impl From<AppState> for Message {
    fn from(a: AppState) -> Self { Message::Switch(a) }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub should_exit:   bool,
    pub fullscreen:    bool,
    pub width:         u32,
    pub height:        u32,
    pub db_location:   PathBuf,
    pub data_location: PathBuf,
    pub lib_action:    Actions,
}
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub enum Actions {
    #[default]
    Move,
    Copy,
    Symlink,
}
impl Default for AppSettings {
    fn default() -> Self {
        Self {
            should_exit:   Default::default(),
            fullscreen:    Default::default(),
            width:         Default::default(),
            height:        Default::default(),
            db_location:   ProjectDirs::from("", "", APP_NAME)
                .expect("Project dirs path not valid")
                .data_local_dir()
                .to_path_buf(),
            data_location: ProjectDirs::from("", "", APP_NAME)
                .expect("Project dirs path not valid")
                .data_dir()
                .to_path_buf(),
            lib_action:    Default::default(),
        }
    }
}

// impl Default for AppSettings {
//     fn default() -> Self {
//         Self {
//             should_exit: Default::default(),
//             fullscreen:  Default::default(),
//             width:       Default::default(),
//             height:      Default::default(),
//         }
//     }
// }

pub trait Styled {
    fn dark(&self) -> (Background, Option<Color>, f32, f32, Color) {
        (WHITE.into(), Some(BLACK), 0., 0., WHITE)
    }
    fn white(&self) -> (Background, Option<Color>, f32, f32, Color) {
        (BLACK.into(), Some(BLACK), 0., 0., BLACK)
    }
    fn style(&self) -> (Background, Option<Color>, f32, f32, Color);
    fn checkbox(&self) -> checkbox::Style {
        let style = self.style();
        checkbox::Style {
            background:      style.0,
            checkmark_color: style.4,
            text_color:      style.1,
            border_radius:   style.2,
            border_width:    style.3,
            border_color:    style.1.unwrap_or_default(),
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub enum Theme {
    Dark,
    White,
}
impl From<bool> for Theme {
    fn from(b: bool) -> Self { if b { Self::Dark } else { Self::White } }
}
impl From<Theme> for Color {
    fn from(theme: Theme) -> Color {
        match theme {
            Theme::Dark => WHITE,
            Theme::White => BLACK,
        }
    }
}
impl checkbox::StyleSheet for Theme {
    fn active(&self, _is_checked: bool) -> checkbox::Style { self.checkbox() }

    fn hovered(&self, _is_checked: bool) -> checkbox::Style { self.checkbox() }
}
impl Styled for Theme {
    fn style(&self) -> (Background, Option<Color>, f32, f32, Color) {
        match self {
            Theme::Dark => self.dark(),
            Theme::White => self.white(),
        }
    }
}
