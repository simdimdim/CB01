use crate::{app::ui::settings::Styled, Message, Theme, ViewA};
use iced::{
    button,
    checkbox,
    Align,
    Button,
    Checkbox,
    Command,
    Container,
    Element,
    HorizontalAlignment,
    Length,
    Row,
    Text,
    VerticalAlignment,
};

#[derive(Debug)]
pub struct SSet {
    pub exitbtn:  button::State,
    pub fs_btn:   button::State,
    pub darkmode: bool,
}
#[derive(Debug, Clone, Copy)]
pub enum ASet {
    ToggleDark(bool),
}
impl SSet {
    pub fn new() -> Self {
        Self {
            exitbtn:  Default::default(),
            fs_btn:   Default::default(),
            darkmode: true,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let checkbox =
            Checkbox::new(self.darkmode, "", |a| ASet::ToggleDark(a).into())
                .spacing(0)
                .size(28);
        // .style(Theme::from(self.darkmode));
        let checkboxtext = Text::new(if self.darkmode { "Night" } else { "Day" })
            .width(Length::Shrink)
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Bottom)
            .color(Theme::from(self.darkmode));
        let exit = Button::new(
            &mut self.exitbtn,
            Text::new("Exit")
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .width(Length::Units(80))
        .min_height(24)
        .on_press(Message::Exit);
        let fs = Button::new(
            &mut self.fs_btn,
            Text::new("Fullscreen")
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .width(Length::Units(120))
        .min_height(24)
        .on_press(Message::FullscreenMode);
        let content = Row::new()
            .align_items(Align::Center)
            .spacing(4)
            .push(checkboxtext)
            .push(checkbox)
            .push(fs)
            .push(exit);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Align::End)
            .align_y(Align::End)
            .into()
    }

    pub fn update(&mut self, message: ASet) -> Command<Message> {
        match message {
            ASet::ToggleDark(b) => self.darkmode = b,

        }
        Command::none()
    }
}
impl From<ASet> for Message {
    fn from(a: ASet) -> Self { Message::Update(ViewA::ASet(a)) }
}
