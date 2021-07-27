use crate::{Message, BLACK, WHITE};
use iced::{checkbox, Background, Color};

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

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub should_exit: bool,
    pub fullscreen:  bool,
    pub width:       u32,
    pub height:      u32,
}
impl Default for AppSettings {
    fn default() -> Self {
        Self {
            should_exit: Default::default(),
            fullscreen:  Default::default(),
            width:       Default::default(),
            height:      Default::default(),
        }
    }
}

pub trait Styled {
    fn dark(&self) -> (Background, Color, f32, f32, Color) {
        (WHITE.into(), BLACK, 0., 0., WHITE)
    }
    fn white(&self) -> (Background, Color, f32, f32, Color) {
        (BLACK.into(), BLACK, 0., 0., BLACK)
    }
    fn style(&self) -> (Background, Color, f32, f32, Color);
    fn checkbox(&self) -> checkbox::Style {
        let style = self.style();
        checkbox::Style {
            background:      style.0,
            border_color:    style.1,
            border_radius:   style.2,
            border_width:    style.3,
            checkmark_color: style.4,
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
    fn style(&self) -> (Background, Color, f32, f32, Color) {
        match self {
            Theme::Dark => self.dark(),
            Theme::White => self.white(),
        }
    }
}
