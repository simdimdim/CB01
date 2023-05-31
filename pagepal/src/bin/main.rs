use iced::{window::Settings as WSettings, Application, Settings};
use pagepal::App;
use simple_logger::SimpleLogger;

pub fn main() -> iced::Result {
	std::env::set_var("RUST_LOG", "warn,retriever=debug");
	env_logger::init();
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
