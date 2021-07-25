use crate::{AppData, Id, Message, ViewA};
use iced::{text_input, Command, Container, Element};

#[derive(Debug, Clone)]
pub struct SBook {
    pub titleinput: text_input::State,
    pub err:        EBook,
    pub show:       Id,
    pub title:      String,
}
#[derive(Debug, Clone)]
pub enum ABook {
    Prev,
    Next,
    View(Id),
    UpdateTitle(String),
}
#[derive(Debug, Clone)]
pub enum EBook {
    No,
}
impl SBook {
    pub fn new() -> Self {
        Self {
            titleinput: text_input::State::new(),
            err:        EBook::No,
            show:       0,
            title:      "".to_owned(),
        }
    }

    pub fn view(&mut self, _data: &mut AppData) -> Element<'_, Message> {
        let title =
            iced::TextInput::new(&mut self.titleinput, "", &self.title, |s| {
                ABook::UpdateTitle(s).into()
            });
        Container::new(title).into()
    }

    pub fn update(
        &mut self, data: &mut AppData, message: ABook,
    ) -> Command<Message> {
        match message {
            ABook::Prev => {}
            ABook::Next => {}
            ABook::View(id) => {
                self.show = id;
                self.title = data.library.titles.title(id).unwrap_or_default().0;
            }
            ABook::UpdateTitle(t) => self.title = t,
        };
        Command::none()
    }
}
impl From<ABook> for Message {
    fn from(a: ABook) -> Self { Message::Update(ViewA::ABook(a)) }
}
