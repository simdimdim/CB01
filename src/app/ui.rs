use crate::{settings::AppState, Library, APP_NAME};
use directories_next::{ProjectDirs, UserDirs};
use iced::{
    executor,
    keyboard::Modifiers,
    window::{
        self,
        Mode::{Fullscreen, Windowed},
    },
    Application,
    Clipboard,
    Color,
    Command,
    Element,
    Subscription,
};
use iced_native::{
    keyboard::{Event::KeyPressed, KeyCode},
    window::Event::{self, CloseRequested},
    Event::{Keyboard, Window},
};
use std::path::PathBuf;

pub mod data;
pub mod screen;
pub mod settings;

pub use self::{data::*, screen::*, settings::*};

#[derive(Debug)]
pub struct App {
    library:  Library,
    data:     AppData,
    settings: AppSettings,
    screens:  Screens,
}
impl App {
    fn new() -> Self {
        Self {
            library:  Library::default(),
            data:     Default::default(),
            settings: Default::default(),
            screens:  Screens::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(iced_native::Event),
    Update(ViewA),
    Switch(AppState),
    FullscreenMode,
    SaveLibrary,
    Exit,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (App, Command<Message>) {
        let mut app = App::new();
        let library = Library::default();
        // TODO: Load library from disc.
        let _fonts = UserDirs::new()
            .as_ref()
            .map(|x| x.font_dir())
            .unwrap()
            .unwrap_or(PathBuf::from("C:\\Windows\\Fonts").as_path());
        let projdirs = ProjectDirs::from("", "", APP_NAME).unwrap();
        let _confdir = projdirs.config_dir();
        app.library = library;
        (app, Command::none())
    }

    fn title(&self) -> String { APP_NAME.to_owned() }

    fn update(
        &mut self, message: Message, clipboard: &mut Clipboard,
    ) -> Command<Message> {
        match &message {
            Message::EventOccurred(event) => match event {
                Keyboard(KeyPressed {
                    key_code: KeyCode::F,
                    modifiers:
                        Modifiers {
                            control: true,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => {
                    self.data.reversed = !self.data.reversed;
                }
                Keyboard(KeyPressed {
                    key_code: KeyCode::D,
                    modifiers:
                        Modifiers {
                            control: true,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => {
                    self.settings.dark = !self.settings.dark;
                }
                Keyboard(KeyPressed {
                    key_code: KeyCode::Home,
                    modifiers:
                        Modifiers {
                            control: false,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => self.screens.update(ViewA::ARead(ARead::Begin)),
                Keyboard(KeyPressed {
                    key_code:
                        KeyCode::Left |
                        KeyCode::Up |
                        KeyCode::PageUp |
                        KeyCode::A |
                        KeyCode::W,

                    modifiers:
                        Modifiers {
                            control: false,
                            shift: false,
                            alt: false,
                            logo: false,
                        },
                }) => self.screens.update(ViewA::ARead(ARead::Prev(
                    self.data.current.len() as f32,
                ))),
                Keyboard(KeyPressed {
                    key_code:
                        KeyCode::Right |
                        KeyCode::Down |
                        KeyCode::PageDown |
                        KeyCode::D |
                        KeyCode::S |
                        KeyCode::Space,
                    modifiers:
                        Modifiers {
                            control: false,
                            shift: false,
                            alt: false,
                            logo: false,
                        },
                }) => self.screens.update(ViewA::ARead(ARead::Next(
                    self.data.current.len() as f32,
                ))),
                Keyboard(KeyPressed {
                    key_code: KeyCode::End,
                    modifiers:
                        Modifiers {
                            control: false,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => self.screens.update(ViewA::ARead(ARead::End)),
                Keyboard(KeyPressed {
                    key_code: KeyCode::NumpadSubtract,
                    modifiers:
                        Modifiers {
                            control: false,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => self.screens.update(ViewA::ARead(ARead::Less)),
                Keyboard(KeyPressed {
                    key_code: KeyCode::NumpadAdd,
                    modifiers:
                        Modifiers {
                            control: false,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => self.screens.update(ViewA::ARead(ARead::More)),
                Keyboard(KeyPressed {
                    key_code: KeyCode::Numpad1,
                    modifiers:
                        Modifiers {
                            control: false,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => {
                    self.update(Message::Switch(AppState::Library), clipboard);
                }
                Keyboard(KeyPressed {
                    key_code: KeyCode::Numpad2,
                    modifiers:
                        Modifiers {
                            control: false,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => {
                    self.update(Message::Switch(AppState::Reader), clipboard);
                }
                Keyboard(KeyPressed {
                    key_code: KeyCode::Numpad3,
                    modifiers:
                        Modifiers {
                            control: false,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => {
                    self.update(Message::Switch(AppState::Settings), clipboard);
                }
                _ => {}
            },
            Message::Exit => {
                self.settings.should_exit = true;
            }
            Message::Switch(state) => {
                self.screens.state = *state;
            }
            Message::Update(a) => self.screens.update(*a),
            Message::FullscreenMode => todo!(),
            Message::SaveLibrary => todo!(),
        }
        handle_settings(&mut self.settings, &message);

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let App { data, settings, .. } = self;
        self.screens.view(data, settings)
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn mode(&self) -> window::Mode {
        match self {
            App { settings, .. } => match settings.fullscreen {
                false => Windowed,
                true => Fullscreen,
            },
        }
    }

    fn background_color(&self) -> Color {
        fn dark(b: bool) -> Color {
            match b {
                true => Color::BLACK,
                false => Color::WHITE,
            }
        }
        match self {
            App { settings, .. } => dark(settings.dark),
        }
    }

    fn should_exit(&self) -> bool {
        match self {
            App { settings, .. } => settings.should_exit,
        }
    }
}

fn handle_settings(settings: &mut AppSettings, message: &Message) {
    match message {
        Message::EventOccurred(event) => match event {
            Window(Event::Resized { width, height }) => {
                settings.width = *width;
                settings.height = *height;
            }
            Window(CloseRequested) |
            Keyboard(KeyPressed {
                key_code: KeyCode::Escape | KeyCode::Q,
                modifiers:
                    Modifiers {
                        control: false,
                        logo: false,
                        shift: false,
                        alt: false,
                    },
            }) => {
                settings.should_exit = true;
            }
            Keyboard(KeyPressed {
                key_code: KeyCode::F | KeyCode::F12,
                modifiers:
                    Modifiers {
                        control: false,
                        logo: false,
                        shift: false,
                        alt: false,
                    },
            }) => {
                settings.fullscreen = !settings.fullscreen;
            }
            _ => {}
        },
        Message::FullscreenMode => {
            settings.fullscreen = !settings.fullscreen;
        }
        Message::Exit => {
            settings.should_exit = true;
        }
        Message::Switch(state) => {
            settings.state = *state;
        }
        Message::SaveLibrary => (),
        Message::Update(_s) => {
            // TODO: Update settings accordingly >.>
        }
    };
}
