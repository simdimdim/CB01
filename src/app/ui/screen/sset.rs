use crate::{AppSettings, Message, Theme, ViewA};
use iced::{
    button,
    Align,
    Button,
    Checkbox,
    Column,
    Command,
    Container,
    Element,
    HorizontalAlignment,
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
            .text_color(Theme::from(self.darkmode).into())
            .width(Length::Shrink)
            .spacing(if self.darkmode { 4 } else { 10 })
            .size(28);

        let fullscreen = Checkbox::new(settings.fullscreen, "Fullscreen", |_| {
            Message::FullscreenMode
        })
        .text_color(Theme::from(self.darkmode).into())
        .width(Length::Shrink)
        .spacing(if self.darkmode { 4 } else { 10 })
        .size(28);

        let darkmode = Checkbox::new(
            self.darkmode,
            if self.darkmode { "Night" } else { "Day" },
            |a| ASet::ToggleDark(a).into(),
        )
        .text_color(Theme::from(self.darkmode).into())
        .width(Length::Shrink)
        .spacing(if self.darkmode { 4 } else { 10 })
        .size(28);

        // .style(Theme::from(self.darkmode));
        let exit = Button::new(
            &mut self.exitbtn,
            Text::new("Exit")
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .width(Length::Shrink)
        .min_height(24)
        .on_press(Message::Exit);

        let middle = Column::new()
            .align_items(Align::Start)
            .width(Length::Shrink)
            .height(Length::Units(settings.height as u16 / 2))
            .spacing(2)
            .push(anywhere)
            .push(fullscreen);
        let main = Row::new()
            .align_items(Align::End)
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(2)
            .push(darkmode)
            .push(Space::new(Length::Fill, Length::Fill))
            .push(middle)
            .push(Space::new(Length::Fill, Length::Fill))
            .push(
                Column::new()
                    .height(Length::Fill)
                    .align_items(Align::Start)
                    .push(exit),
            );
        Container::new(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Align::Center)
            .align_x(Align::End)
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
