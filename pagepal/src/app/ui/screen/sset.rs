use crate::{AppSettings, Message, Theme, ViewA};
use iced::{
    alignment::{Horizontal, Vertical},
    button,
    Alignment,
    Button,
    Checkbox,
    Column,
    Command,
    Container,
    Element,
    Length,
    Row,
    Space,
    Text,
};

#[derive(Debug)]
pub struct SSet {
    pub exitbtn:  button::State,
    pub fs_btn:   button::State,
    pub darkmode: bool,
    pub anywhere: bool,
}
#[derive(Debug, Clone, Copy)]
pub enum ASet {
    ToggleDark(bool),
    ToggleAnywhere(bool),
}
impl SSet {
    pub fn new() -> Self {
        Self {
            exitbtn:  Default::default(),
            fs_btn:   Default::default(),
            darkmode: true,
            anywhere: true,
        }
    }

    pub fn view(&mut self, settings: &mut AppSettings) -> Element<'_, Message> {
        let anywhere =
            Checkbox::new(self.anywhere, "Paste links from any screen", |a| {
                ASet::ToggleAnywhere(a).into()
            })
            .style(Theme::from(self.darkmode))
            .width(Length::Shrink)
            .spacing(if self.darkmode { 4 } else { 10 })
            .size(28);
        let fullscreen = Checkbox::new(settings.fullscreen, "Fullscreen", |_| {
            Message::FullscreenMode
        })
        .width(Length::Shrink)
        .spacing(if self.darkmode { 4 } else { 10 })
        .size(28);

        let darkmode = Checkbox::new(
            self.darkmode,
            if self.darkmode { "Night" } else { "Day" },
            |a| ASet::ToggleDark(a).into(),
        )
        .style(Theme::from(self.darkmode))
        .width(Length::Shrink)
        .spacing(if self.darkmode { 4 } else { 10 })
        .size(28);

        let exit = Button::new(
            &mut self.exitbtn,
            Text::new("Exit")
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(Length::Shrink)
                    //                    .min_height(24)
        .on_press(Message::Exit);

        let middle = Column::new()
            .align_items(Alignment::Start)
            .width(Length::Shrink)
            .height(Length::Units(settings.height as u16 / 2))
            .spacing(2)
            .push(anywhere)
            .push(fullscreen);
        let main = Row::new()
            .align_items(Alignment::End)
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(2)
            .push(darkmode)
            .push(Space::new(Length::Fill, Length::Fill))
            .push(middle)
            .push(Space::new(Length::Fill, Length::Fill))
            .push(
                Column::new()
                    .height(Length::Units(24))
                    .align_items(Alignment::Start)
                    .push(exit),
            );
        Container::new(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Vertical::Center)
            .align_x(Horizontal::Right)
            .into()
    }

    pub fn update(&mut self, message: ASet) -> Command<Message> {
        match message {
            ASet::ToggleDark(b) => self.darkmode = b,
            ASet::ToggleAnywhere(b) => self.darkmode = b,
        }
        Command::none()
    }
}
impl From<ASet> for Message {
    fn from(a: ASet) -> Self { Message::Update(ViewA::ASet(a)) }
}
