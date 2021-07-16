use crate::{ASet, AppData, Book, Id, Message, ViewA};
use iced::{
    pane_grid::{Axis, Content, State},
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
    pub panes: Option<State<Book>>,
}
#[derive(Debug, Clone, Copy)]
pub enum ALib {
    Select(Id),
    Swap(Id, Id),
    Dragged(iced::pane_grid::DragEvent),
    Split(()),
}

impl SLib {
    pub fn new() -> Self { Self { panes: None } }

    pub fn view(&mut self, data: &mut AppData) -> Element<Message> {
        let l = data.library.group_size("Reading");
        let g = data.library.get_group("Reading");
        let (mut panes, first) = State::new(data.library.current().clone());
        g.unwrap().into_iter().for_each(|b| {
            panes.split(Axis::Horizontal, &first, b);
        });
        self.panes = Some(panes);
        let pane_grid = PaneGrid::new(
            self.panes.as_mut().unwrap(),
            |_pane, b| -> Content<Message> { b.view().into() },
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_drag(|a| ALib::Dragged(a).into());
        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: ALib) -> Command<Message> {
        match message {
            ALib::Select(_) => (),
            ALib::Swap(..) => (),
            ALib::Dragged(iced::pane_grid::DragEvent::Dropped {
                pane: _,
                target: _,
            }) => {
                // self.libview.panes.swap(&pane, &target);
            }
            ALib::Dragged(_) => {}
            ALib::Split(()) => (),
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
