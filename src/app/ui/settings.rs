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

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub should_exit: bool,
    pub fullscreen:  bool,
    pub dark:        bool,
    pub width:       u32,
    pub height:      u32,
    pub columns:     u16,
    pub state:       AppState,
}
impl Default for AppSettings {
    fn default() -> Self {
        Self {
            state:       Default::default(),
            should_exit: Default::default(),
            fullscreen:  Default::default(),
            dark:        true,
            width:       Default::default(),
            height:      Default::default(),
            columns:     1,
        }
    }
}
