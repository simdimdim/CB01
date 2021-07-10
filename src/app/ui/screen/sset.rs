use crate::{  Message, ViewA};
use iced::{
    button,
    Align,
    Button,
    Checkbox,
    Container,
    Element,
    HorizontalAlignment,
    Length,
    Row,
    Text,
};

pub struct SSet {
    pub exitbtn:  button::State,
    pub fs_btn:   button::State,
    pub darkmode: Checkbox<Message>,
    darkmodebool: bool,
}
#[derive(Debug, Clone, Copy)]
pub enum ASet {
    ToggleDark(bool),
    Dragged(iced::pane_grid::DragEvent),
}
impl SSet {
    pub fn new() -> Self {
        let darkmodebool = false;
        Self {
            exitbtn: Default::default(),
            fs_btn: Default::default(),
            darkmode: Checkbox::new(darkmodebool, "Dark mode", |a| {
                Message::Update(ViewA::ASet(ASet::ToggleDark(a)))
            }),
            darkmodebool,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let checkbox = Checkbox::new(self.darkmodebool, "Dark mode", |a| {
            Message::Update(ViewA::ASet(ASet::ToggleDark(a)))
        })
        .width(Length::Fill);
        let exit = Button::new(
            &mut self.exitbtn,
            Text::new("Exit")
                .width(Length::Fill)
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .width(Length::Units(80))
        .padding(8)
        .on_press(Message::Exit);
        let fs = Button::new(
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

    pub fn update(&mut self, message: ASet) {
        match message {
            ASet::ToggleDark(b) => self.darkmodebool = b,
            ASet::Dragged(iced::pane_grid::DragEvent::Dropped {
                pane: _,
                target: _,
            }) => {
                // self.libview.panes.swap(&pane, &target);
            }
            ASet::Dragged(_) => {}
        }
    }
}

impl std::fmt::Debug for SSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.darkmodebool).finish()
    }
}
