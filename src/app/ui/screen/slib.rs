use crate::{AppData, Book, Id, Label, Message, ViewA};
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
    pub panes: Option<State<(Label, Book)>>,
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

    pub fn view(&mut self, data: &mut AppData) -> Element<'_, Message> {
        let _l = data.library.group_size("Reading");
        let g = data
            .library
            .get_group_names("Reading")
            .unwrap()
            .into_iter()
            .zip(data.library.get_group("Reading").unwrap().into_iter());
        let (mut panes, first) = State::new({
            let (&id, book) = data.library.books.iter().next().unwrap();
            (data.library.titles.title(id).unwrap(), book.to_owned())
        });
        g.into_iter().for_each(|(t, b)| {
            panes.split(Axis::Horizontal, &first, (t, b));
        });
        self.panes = Some(panes);
        let pane_grid = PaneGrid::new(
            self.panes.as_mut().unwrap(),
            |_pane, b| -> Content<'_, Message> {
                let content =
                    Text::new(format!("{}", *b.0)).size(32).color(Color {
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
            },
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .on_click(|_a| ALib::Select(1).into())
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

impl From<ALib> for Message {
    fn from(a: ALib) -> Self { Message::Update(ViewA::ALib(a)) }
}
