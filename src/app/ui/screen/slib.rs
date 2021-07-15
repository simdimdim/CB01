use crate::{ASet, Book, Id, Message, ViewA};
use iced::{
    pane_grid::{Content, State},
    Color,
    Command,
    Container,
    Element,
    Length,
    PaneGrid,
    Text,
};

#[derive(Debug)]
pub struct SLib {
    pub panes: State<Book>,
}
#[derive(Debug, Clone, Copy)]
pub enum ALib {
    Select(Id),
    Swap(Id, Id),
}

impl SLib {
    pub fn new() -> Self {
        let (panes, _) = State::new(Book::default());
        Self { panes }
    }

    pub fn view(&mut self) -> Element<Message> {
        let pane_grid = PaneGrid::new(
            &mut self.panes,
            |_pane, b: &mut Book| -> Content<Message> { b.view().into() },
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_drag(|a| ASet::Dragged(a).into());
        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: ALib) -> Command<Message> {
        match message {
            ALib::Select(_) => (),
            ALib::Swap(..) => (),
        }
        Command::none()
    }
}
impl Book {
    pub fn view(&mut self) -> Element<Message> {
        let content = Text::new(format!("Book {}", self.chapters[0].offset))
            .size(32)
            .color(Color {
                r: 0.,
                g: 255.,
                b: 0.,
                a: 1.,
            });
        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
impl From<ALib> for Message {
    fn from(a: ALib) -> Self { Message::Update(ViewA::ALib(a)) }
}
