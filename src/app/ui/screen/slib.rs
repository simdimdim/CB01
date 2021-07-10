use crate::{ASet, Book, Id, Message, ViewA};
use iced::{
    pane_grid::{Content, State},
    Color,
    Container,
    Element,
    Length,
    PaneGrid,
    Text,
};

#[derive(Debug)]
pub struct SLib {
    pub panes:    State<Book>,
    pub modified: bool,
}
#[derive(Debug, Clone, Copy)]
pub enum ALib {
    Select(Id),
}

impl SLib {
    pub fn new() -> Self {
        let (panes, _) = State::new(Book::default());
        Self {
            panes,
            modified: false,
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let pane_grid = PaneGrid::new(
            &mut self.panes,
            |_pane, b: &mut Book| -> Content<Message> { b.view().into() },
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_drag(|a| Message::Update(ViewA::ASet(ASet::Dragged(a))));
        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: ALib) {
        match message {
            ALib::Select(_) => todo!(),
        }
    }
}
impl Book {
    pub fn view(&mut self) -> Element<Message> {
        let content =
            Text::new(format!("Book {}", self.id))
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