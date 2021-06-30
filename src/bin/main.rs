use iced::{
    button,
    executor,
    window,
    Align,
    Application,
    Button,
    Clipboard,
    Color,
    Command,
    Container,
    Element,
    HorizontalAlignment,
    Length,
    Row,
    Settings,
    Subscription,
    Text,
};
use iced_native::{
    keyboard::{Event::KeyPressed, KeyCode},
    window::Event,
    Event as NativeEvent,
};
use pagepal_ui::APP_NAME;
use window::Mode::{Fullscreen, Windowed};
use Event::CloseRequested;
use NativeEvent::{Keyboard, Window};

pub fn main() -> iced::Result {
    AppData::run(Settings {
        antialiasing: true,
        exit_on_close_request: true,
        ..Settings::default()
    })
}

#[derive(Debug, Default)]
struct AppData {
    exitbtn:     button::State,
    fs_btn:      button::State,
    should_exit: bool,
    fullscreen:  bool,
}
#[derive(Debug, Clone)]
enum Message {
    EventOccurred(iced_native::Event),
    FullscreenMode,
    Exit,
}

impl Application for AppData {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: ()) -> (AppData, Command<Message>) {
        (AppData::default(), Command::none())
    }

    fn title(&self) -> String { APP_NAME.to_owned() }

    fn update(
        &mut self, message: Message, _clipboard: &mut Clipboard,
    ) -> Command<Message> {
        match message {
            Message::EventOccurred(event) => match event {
                Window(CloseRequested) => {
                    self.should_exit = true;
                }
                Keyboard(KeyPressed {
                    key_code: KeyCode::F,
                    ..
                }) => {
                    self.fullscreen = !self.fullscreen;
                }
                Keyboard(KeyPressed {
                    key_code: KeyCode::Q,
                    ..
                }) => {
                    self.should_exit = true;
                }
                _ => {}
            },
            Message::FullscreenMode => {
                self.fullscreen = !self.fullscreen;
            }
            Message::Exit => {
                self.should_exit = true;
            }
        };

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced_native::subscription::events().map(Message::EventOccurred)
    }

    fn should_exit(&self) -> bool { self.should_exit }

    fn view(&mut self) -> Element<Message> {
        let exit = Button::new(
            &mut self.exitbtn,
            Text::new("Exit")
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .width(Length::Units(80))
        .padding(8)
        .on_press(Message::Exit);

        let fullscreen = Button::new(
            &mut self.fs_btn,
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
            .push(fullscreen)
            .push(exit);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Align::End)
            .align_y(Align::End)
            .into()
    }

    fn background_color(&self) -> Color { Color::BLACK }

    fn mode(&self) -> window::Mode {
        match self.fullscreen {
            false => Windowed,
            true => Fullscreen,
        }
    }
}
