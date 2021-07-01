use iced::{Application, Settings};
use pagepal_ui::App;

pub fn main() -> iced::Result {
    App::run(Settings {
        antialiasing: true,
        exit_on_close_request: true,
        ..Settings::default()
    })
}
