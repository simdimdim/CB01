#![allow(unused_imports)]

use self::{data::AppData, settings::AppSettings};
use crate::{settings::AppState, APP_NAME};
use iced::{
    button,
    executor,
    image,
    image::Viewer,
    keyboard::Modifiers,
    scrollable,
    window,
    Align,
    Application,
    Button,
    Clipboard,
    Color,
    Column,
    Command,
    Container,
    Element,
    HorizontalAlignment,
    Image,
    Length,
    Rectangle,
    Row,
    Scrollable,
    Size,
    Subscription,
    Text,
    VerticalAlignment,
};
use iced_native::{
    keyboard::{Event::KeyPressed, KeyCode},
    window::Event,
    Event as NativeEvent,
};
use image::viewer;
use itertools::{Chunk, Either, Itertools};
use std::cell::{Cell, RefCell};
use window::Mode::{Fullscreen, Windowed};
use Event::CloseRequested;
use NativeEvent::{Keyboard, Window};

pub mod content;
pub mod data;
pub mod settings;

pub use self::content::*;

// #[derive(Debug)]
// pub struct State {
//     app:   App,
//     state: AppState,
// }

#[derive(Debug)]
pub struct App {
    scroff:   f32,
    scroll:   scrollable::State,
    data:     AppData,
    settings: AppSettings,
}
impl App {
    fn new() -> Self {
        Self {
            scroff:   0f32,
            scroll:   scrollable::State::new(),
            data:     Default::default(),
            settings: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    EventOccurred(iced_native::Event),
    Scrolled(f32),
    SwitchToSettings,
    SwitchToReader,
    SwitchToLibrary,
    FullscreenMode,
    Exit,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (App, Command<Message>) {
        (App::new(), Command::none())
    }

    fn title(&self) -> String { APP_NAME.to_owned() }

    fn update(
        &mut self, message: Message, _clipboard: &mut Clipboard,
    ) -> Command<Message> {
        match self {
            App {
                scroff: offset,
                scroll,
                settings,
                data,
            } => {
                match &message {
                    Message::EventOccurred(event) => match event {
                        Window(CloseRequested) => {
                            settings.should_exit = true;
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
                            *offset = (*offset -
                                (data.current.len() as f32 /
                                    settings.columns as f32 -
                                    1.)
                                .recip())
                            .max(0.);
                            scroll.snap_to(*offset);
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
                            *offset = (*offset +
                                (data.current.len() as f32 /
                                    settings.columns as f32 -
                                    1.)
                                .recip())
                            .min(1.);
                            scroll.snap_to(*offset);
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
                            data.reversed = !data.reversed;
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
                            *offset = 0.;
                            scroll.snap_to(*offset);
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
                            *offset = 1.;
                            scroll.snap_to(*offset);
                        }
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
                            settings.state = AppState::Library;
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
                            settings.state = AppState::Reader;
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
                            settings.state = AppState::Settings;
                        }
                        _ => {}
                    },
                    Message::Exit => {
                        settings.should_exit = true;
                    }
                    Message::Scrolled(off) => {
                        *offset = *off;
                    }
                    Message::SwitchToSettings => {
                        settings.state = AppState::Settings;
                    }
                    Message::SwitchToReader => {
                        settings.state = AppState::Reader;
                    }
                    Message::SwitchToLibrary => {
                        settings.state = AppState::Library;
                    }
                    _ => (),
                }
                handle_settings(settings, &message);
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            App {
                scroll,
                data,
                settings:
                    settings
                    @
                    AppSettings {
                        state: AppState::Reader,
                        ..
                    },
                ..
            } => draw_reader(scroll, data, settings),
            App {
                settings:
                    settings
                    @
                    AppSettings {
                        state: AppState::Library,
                        ..
                    },
                ..
            } => draw_library(settings),
            App {
                settings:
                    settings
                    @
                    AppSettings {
                        state: AppState::Settings,
                        ..
                    },
                ..
            } => draw_settings(settings),
        }
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

fn handle_settings(settings: &mut AppSettings, _message: &Message) {
    match _message {
        Message::EventOccurred(event) => match event {
            Window(Event::Resized { width, height }) => {
                settings.width = *width;
                settings.height = *height;
            }
            Window(CloseRequested) => {
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
            Keyboard(KeyPressed {
                key_code: KeyCode::Escape,
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
                key_code: KeyCode::NumpadAdd,
                modifiers:
                    Modifiers {
                        control: false,
                        logo: false,
                        shift: false,
                        alt: false,
                    },
            }) => {
                settings.columns = settings.columns.saturating_add(1);
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
                settings.columns = 1.max(settings.columns.saturating_sub(1));
            }
            _ => {}
        },
        Message::FullscreenMode => {
            settings.fullscreen = !settings.fullscreen;
        }
        Message::Exit => {
            settings.should_exit = true;
        }
        Message::Scrolled(_) => {}
        Message::SwitchToSettings => {
            settings.state = AppState::Settings;
        }
        Message::SwitchToReader => {
            settings.state = AppState::Reader;
        }
        Message::SwitchToLibrary => {
            settings.state = AppState::Library;
        }
    };
}

fn draw_settings(settings: &mut AppSettings) -> Element<Message> {
    let exit = Button::new(
        &mut settings.exitbtn,
        Text::new("Exit")
            .width(Length::Fill)
            .horizontal_alignment(HorizontalAlignment::Center),
    )
    .width(Length::Units(80))
    .padding(8)
    .on_press(Message::Exit);
    let fs = Button::new(
        &mut settings.fs_btn,
        Text::new("Fullscreen")
            .width(Length::Fill)
            .horizontal_alignment(HorizontalAlignment::Center),
    )
    .width(Length::Units(120))
    .padding(8)
    .on_press(Message::FullscreenMode);
    let content = Row::new()
        .align_items(Align::Center)
        .spacing(4)
        .push(fs)
        .push(exit);

    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Align::End)
        .align_y(Align::End)
        .into()
}
fn draw_library(settings: &mut AppSettings) -> Element<Message> {
    let exit = Button::new(
        &mut settings.exitbtn,
        Text::new("Exit")
            .width(Length::Fill)
            .horizontal_alignment(HorizontalAlignment::Center),
    )
    .width(Length::Units(80))
    .padding(8)
    .on_press(Message::Exit);
    let content = Row::new().align_items(Align::Center).spacing(4).push(exit);
    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Align::End)
        .align_y(Align::End)
        .into()
}
fn draw_reader<'a>(
    scroll: &'a mut scrollable::State, data: &mut AppData,
    settings: &mut AppSettings,
) -> Element<'a, Message> {
    let re = data.reversed;
    let cn = data
        .current
        // TODO: skin n take, chunk
        .chunks_mut(settings.columns.max(1) as usize)
        .fold(
            Scrollable::<'a, Message>::new(scroll)
                .align_items(Align::Center)
                .on_scroll(move |off| Message::Scrolled(off)),
            |mut content, ch| {
                if re {
                    ch.reverse();
                }
                content = content
                    .push(ch.into_iter().fold(
                        Row::<'a, Message>::new().align_items(Align::Center),
                        |mut row, cnt| {
                            let elem: Element<'_, Message> = match cnt {
                                Content::Image(z) => Image::new(z.clone())
                                    .width(Length::FillPortion(settings.columns))
                                    .height(Length::FillPortion(settings.columns))
                                    .into(),
                                Content::Text(z) => Text::new(z.clone())
                                    .width(Length::Fill)
                                    .vertical_alignment(VerticalAlignment::Top)
                                    .horizontal_alignment(
                                        HorizontalAlignment::Center,
                                    )
                                    .into(),
                            };
                            row = row
                                .push(elem)
                                .max_width(settings.width)
                                .max_height(settings.height);
                            row
                        },
                    ))
                    .max_width(settings.width);
                content
            },
        );
    if re {
        data.reversed = !data.reversed;
    }
    Container::new(cn)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Align::Center)
        .align_y(Align::Center)
        .into()
}
