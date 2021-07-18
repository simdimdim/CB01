use crate::{Library, APP_NAME};
use directories_next::{ProjectDirs, UserDirs};
use iced::{
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
    executor::Tokio,
    keyboard::{Event::KeyPressed, KeyCode},
    window::Event::{self, CloseRequested},
    Event::{Keyboard, Window},
};
use log::warn;
use std::path::PathBuf;

pub mod data;
pub mod screen;
pub mod settings;

pub use self::{data::*, screen::*, settings::*};

pub struct App {
    data:     AppData,
    settings: AppSettings,
    screens:  Screens,
}
impl App {
    fn new() -> Self {
        Self {
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
    type Executor = Tokio;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (App, Command<Message>) {
        let mut app = App::new();
        let library = Library::default();
        // TODO: Load library from disc.
        #[allow(clippy::or_fun_call)]
        let _fonts = UserDirs::new()
            .as_ref()
            .map(|x| x.font_dir())
            .unwrap()
            .unwrap_or(PathBuf::from("C:\\Windows\\Fonts").as_path());
        let projdirs = ProjectDirs::from("", "", APP_NAME).unwrap();
        let _confdir = projdirs.config_dir();
        app.data.library = library;
        (app, Command::none())
    }

    fn title(&self) -> String { APP_NAME.to_owned() }

    fn update(
        &mut self, message: Message, clipboard: &mut Clipboard,
    ) -> Command<Message> {
        handle_settings(&mut self.settings, &message);
        match message {
            Message::EventOccurred(event) => match event {
                Keyboard(KeyPressed {
                    key_code: KeyCode::V,
                    modifiers:
                        Modifiers {
                            control: true,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) if self.screens.sset.anywhere => {
                    self.screens.state = AppState::Add;
                    if let Some(s) = clipboard.read() {
                        return Command::perform(async {}, move |_| {
                            AAdd::Fetch(
                                s.parse()
                                    .map_err(|e| {
                                        warn!("{} : {}", e, s);
                                    })
                                    .unwrap_or_else(|_| {
                                        "htpps://codenova.ddns.net"
                                            .parse()
                                            .unwrap()
                                    }),
                            )
                            .into()
                        });
                    }
                }
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
                    self.screens.sread.rev = !self.screens.sread.rev;
                }
                Keyboard(KeyPressed {
                    key_code: KeyCode::S,
                    modifiers:
                        Modifiers {
                            control: true,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => {
                    self.screens.sread.single = !self.screens.sread.single;
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
                    self.update(
                        ASet::ToggleDark(!self.screens.sset.darkmode).into(),
                        clipboard,
                    );
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
                }) => {
                    self.update(ARead::Begin.into(), clipboard);
                }
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
                }) => {
                    self.update(ARead::Prev.into(), clipboard);
                }
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
                }) => {
                    self.update(ARead::Next.into(), clipboard);
                }
                Keyboard(KeyPressed {
                    key_code: KeyCode::End,
                    modifiers:
                        Modifiers {
                            control: false,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => {
                    self.update(ARead::End.into(), clipboard);
                }
                Keyboard(KeyPressed {
                    key_code: KeyCode::NumpadSubtract,
                    modifiers:
                        Modifiers {
                            control: false,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => {
                    self.update(ARead::Less.into(), clipboard);
                }
                Keyboard(KeyPressed {
                    key_code: KeyCode::NumpadAdd,
                    modifiers:
                        Modifiers {
                            control: false,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => {
                    self.update(ARead::More.into(), clipboard);
                }
                Keyboard(KeyPressed {
                    key_code: KeyCode::F1 | KeyCode::Numpad0,
                    modifiers:
                        Modifiers {
                            control: false,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => {
                    self.update(AppState::Settings.into(), clipboard);
                }
                Keyboard(KeyPressed {
                    key_code: KeyCode::F2 | KeyCode::Numpad1,
                    modifiers:
                        Modifiers {
                            control: false,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => {
                    self.update(AppState::Library.into(), clipboard);
                }
                Keyboard(KeyPressed {
                    key_code: KeyCode::F3 | KeyCode::Numpad2,
                    modifiers:
                        Modifiers {
                            control: false,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => {
                    self.update(AppState::Reader.into(), clipboard);
                }
                Keyboard(KeyPressed {
                    key_code: KeyCode::F4 | KeyCode::Numpad3,
                    modifiers:
                        Modifiers {
                            control: false,
                            logo: false,
                            shift: false,
                            alt: false,
                        },
                }) => {
                    self.update(AppState::Add.into(), clipboard);
                }
                _ => {}
            },
            Message::Exit => {
                self.settings.should_exit = true;
            }
            Message::Switch(state) => {
                self.screens.state = state;
            }
            Message::Update(a) => {
                return self.screens.update(&mut self.data, &self.settings, a);
            }
            Message::FullscreenMode => {
                self.settings.fullscreen = !self.settings.fullscreen
            }
            Message::SaveLibrary => todo!(),
        };

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Message> {
        let App { data, settings, .. } = self;
        self.screens.view(data, settings)
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn mode(&self) -> window::Mode {
        match self.settings.fullscreen {
            false => Windowed,
            true => Fullscreen,
        }
    }

    fn background_color(&self) -> Color {
        match self.screens.sset.darkmode {
            true => Color::BLACK,
            false => Color::WHITE,
        }
    }

    fn should_exit(&self) -> bool { self.settings.should_exit }
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
        Message::Exit => {
            settings.should_exit = true;
        }
        _ => (),
    };
}
