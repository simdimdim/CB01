use crate::{AppData, Id, Message, ViewA};
use iced::{
    pick_list,
    text_input,
    Align,
    Column,
    Command,
    Container,
    Element,
    Length,
    PickList,
    Row,
    Space,
};

#[derive(Debug, Clone)]
pub struct SBook {
    pub titlein: text_input::State,
    pub pick:    pick_list::State<String>,
    pub err:     EBook,
    pub show:    Option<Id>,
    pub title:   String,
}
#[derive(Debug, Clone)]
pub enum ABook {
    Prev,
    Next,
    // View(Id),
    View(String),
    UpdateTitle(String),
}
#[derive(Debug, Clone)]
pub enum EBook {
    No,
}
impl SBook {
    pub fn new() -> Self {
        Self {
            titlein: text_input::State::default(),
            pick:    pick_list::State::default(),
            err:     EBook::No,
            show:    None,
            title:   "".to_owned(),
        }
    }

    pub fn view(
        &mut self, data: &mut AppData, _darkmode: bool,
    ) -> Element<'_, Message> {
        let mut v = data.library.titles.find_all(self.title.as_str());
        if let Some(pos) = v.iter().position(|x| x == &self.title) {
            v.remove(pos);
        };
        let title =
            iced::TextInput::new(&mut self.titlein, "", &self.title, |s| {
                ABook::UpdateTitle(s).into()
            });
        let mut main = Column::new().align_items(Align::Center).push(
            Row::new()
                .align_items(Align::Start)
                .width(Length::Fill)
                .push(Space::new(Length::Fill, Length::Shrink))
                .push(title)
                .push(Space::new(Length::Fill, Length::Shrink)),
        );
        if !v.is_empty() {
            let select =
                PickList::new(&mut self.pick, v, None, |s| ABook::View(s).into());
            main = main.push(select);
        }
        Container::new(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Align::Center)
            .align_y(Align::Center)
            .into()
    }

    pub fn update(
        &mut self, data: &mut AppData, message: ABook,
    ) -> Command<Message> {
        match message {
            ABook::Prev => {}
            ABook::Next => {}
            ABook::View(title) => {
                if let Some(id) = data.library.titles.id(&title.into()).copied() {
                    self.show = Some(id);
                    self.title =
                        data.library.titles.title(id).unwrap_or_default().0;
                }
            }
            ABook::UpdateTitle(t) => self.title = t,
        };
        Command::none()
    }
}
impl From<ABook> for Message {
    fn from(a: ABook) -> Self { Message::Update(ViewA::ABook(a)) }
}
