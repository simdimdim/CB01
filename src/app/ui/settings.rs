use crate::Message;

#[derive(Debug, Copy, Clone)]
pub enum AppState {
    Settings,
    Reader,
    Library,
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
    pub should_exit:     bool,
    pub fullscreen:      bool,
    pub dark:            bool,
    pub width:           u32,
    pub height:          u32,
    pub columns:         u16,
    pub state:           AppState,
    pub addfromanywhere: bool,
}
impl Default for AppSettings {
    fn default() -> Self {
        Self {
            should_exit:     Default::default(),
            fullscreen:      Default::default(),
            dark:            true,
            width:           Default::default(),
            height:          Default::default(),
            columns:         1,
            state:           Default::default(),
            addfromanywhere: true,
        }
    }
}
