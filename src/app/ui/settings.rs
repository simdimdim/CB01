use iced::button;

#[derive(Debug)]
pub enum AppState {
    Settings,
    Reader,
    Library,
}
impl Default for AppState {
    fn default() -> Self { Self::Library }
}

#[derive(Debug)]
pub struct AppSettings {
    pub exitbtn:     button::State,
    pub fs_btn:      button::State,
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
            exitbtn:     Default::default(),
            fs_btn:      Default::default(),
            should_exit: Default::default(),
            fullscreen:  Default::default(),
            dark:        true,
            width:       Default::default(),
            height:      Default::default(),
            columns:     1,
        }
    }
}
