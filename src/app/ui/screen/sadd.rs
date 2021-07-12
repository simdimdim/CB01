use crate::{data::AppData, AppSettings, Book, Id, Label, Message, ViewA};
use iced::{
    button,
    Align,
    Button,
    Color,
    Column,
    Command,
    Container,
    Element,
    Length,
    Text,
};
use reqwest::Url;

#[derive(Debug)]
pub struct SAdd {
    pub addbtn: button::State,
    pub title:  String,
    pub book:   Option<Book>,
}
#[derive(Debug, Clone)]
pub enum AAdd {
    Add(Id),
    Fetch(Url),
    UpdateTitle(Label),
    UpdateBook(Label, Book),
    Refresh(Id),
    Loading,
}

impl SAdd {
    pub fn new() -> Self {
        Self {
            addbtn: Default::default(),
            title:  Default::default(),
            book:   None,
        }
    }

    pub fn view(&mut self, _data: &mut AppData) -> Element<Message> {
        let title = Text::new(&self.title).size(32).color(Color {
            r: 0.,
            g: 255.,
            b: 0.,
            a: 1.,
        });
        let addbtn =
            Button::new(&mut self.addbtn, Text::new("Add").width(Length::Fill))
                .width(Length::Units(80))
                .on_press(
                    AAdd::Fetch(
                        "https://zinmanga.com/manga/first-miss-reborn/chapter-1/"
                            .parse()
                            .unwrap(),
                    )
                    .into(),
                );
        let col = Column::new()
            .align_items(Align::Center)
            .push(title)
            .push(addbtn);
        Container::new(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Align::Center)
            .align_y(Align::Center)
            .into()
    }

    pub fn update(
        &mut self, data: &mut AppData, message: AAdd,
    ) -> Command<Message> {
        match message {
            AAdd::Fetch(a) => {
                let r = data.retriever.clone();
                return Command::perform(
                    async move { r.new_book(a).await },
                    move |(title, book)| AAdd::UpdateBook(title, book).into(),
                );
            }
            AAdd::Add(_) => todo!(),
            AAdd::UpdateTitle(t) => self.title = t.0,
            AAdd::UpdateBook(t, b) => {
                self.title = t.0;
                self.book = Some(b);
            }
            AAdd::Refresh(_) => todo!(),
            AAdd::Loading => todo!(),
        }
        Command::none()
    }
}
impl From<AAdd> for Message {
    fn from(a: AAdd) -> Self { Message::Update(ViewA::AAdd(a)) }
}
