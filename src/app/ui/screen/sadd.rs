use crate::{
    data::AppData,
    AppSettings,
    AppState,
    Book,
    Id,
    Label,
    Message,
    ViewA,
};
use iced::{
    button,
    Align,
    Button,
    Checkbox,
    Color,
    Column,
    Command,
    Container,
    Element,
    Length,
    Text,
    VerticalAlignment,
};
use reqwest::Url;

pub struct SAdd {
    pub follow: bool,
    pub addbtn: button::State,
    pub title:  Label,
    pub err:    String,
    pub book:   Option<Book>,
}
impl Default for SAdd {
    fn default() -> Self {
        Self {
            follow: false,
            addbtn: Default::default(),
            title:  Default::default(),
            err:    Default::default(),
            book:   None,
        }
    }
}
#[derive(Debug, Clone)]
pub enum AAdd {
    AddBook(Book),
    Fetch(Url),
    UpdateTitle(Label),
    UpdateBook(Label, Book),
    Refresh(Id),
    ToggleFollow(bool),
}

impl<'a> SAdd {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let err = Text::new(&self.err).size(20).color(Color {
            r: 200.,
            g: 0.,
            b: 0.,
            a: 1.,
        });
        let title = Text::new(&self.title.0).size(32).color(Color {
            r: 0.,
            g: 255.,
            b: 0.,
            a: 1.,
        });
        let follow = Checkbox::new(self.follow, "To reader on add.", |a| {
            AAdd::ToggleFollow(a).into()
        });
        let addbtn = Button::new(
            &mut self.addbtn,
            Text::new(if self.book.is_none() {
                "Fetch"
            } else {
                "Add to lib"
            })
            .vertical_alignment(VerticalAlignment::Center)
            .horizontal_alignment(iced::HorizontalAlignment::Center),
        )
        .width(Length::Shrink)
        .on_press(if self.book.is_none() {
            AAdd::Fetch(
                "https://zinmanga.com/manga/first-miss-reborn/chapter-1/"
                    .parse()
                    .unwrap(),
            )
            .into()
        } else {
            AAdd::AddBook(self.book.as_ref().unwrap().to_owned()).into()
        });
        let col = Column::new()
            .align_items(Align::Center)
            .push(err)
            .push(title)
            .push(follow)
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
            AAdd::AddBook(bk) => {
                self.title = "".into();
                self.book = data.library.add_book(&self.title, bk);
                if self.follow {
                    data.library.cur =
                        data.library.titles.id(&self.title).unwrap();
                    return Command::perform(async {}, |_| {
                        AppState::Reader.into()
                    });
                }
            }
            AAdd::UpdateTitle(t) => self.title = t,
            AAdd::UpdateBook(t, b) => {
                self.title = t;
                self.book = Some(b);
            }
            AAdd::Refresh(_) => todo!(),
            AAdd::ToggleFollow(b) => self.follow = b,
        }
        Command::none()
    }
}
impl From<AAdd> for Message {
    fn from(a: AAdd) -> Self { Message::Update(ViewA::AAdd(a)) }
}
