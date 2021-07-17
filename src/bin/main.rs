use iced::{window::Settings as WSettings, Application, Settings};
use pagepal_ui::App;

pub fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        exit_on_close_request: true,
        window: WSettings {
            decorations: false,
            transparent: true,
            size: (1, 1),
            ..Default::default()
        },
        ..Settings::default()
    })
}
