use crate::{Book, Message};
use iced::{pane_grid::State, Color, Container, Element, Length, PaneGrid, Text};

#[derive(Debug, Clone)]
pub struct LibView {
    pub panes: State<Book>,
}

impl LibView {
    pub fn new() -> Self {
        let (panes, _) = State::new(Book::default());
        Self { panes }
    }

    pub fn view(&mut self) -> Element<Message> {
        let _total_panes = self.panes.len();
        let pane_grid = PaneGrid::new(&mut self.panes, |_pane, book| {
            let content =
                Text::new(format!("Book {}", book.id))
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
                .into()
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .on_drag(Message::Dragged);
        Container::new(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
impl Default for LibView {
    fn default() -> Self { Self::new() }
}
