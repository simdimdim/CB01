use crate::{
    data::AppData,
    ALib,
    AppSettings,
    AppState,
    Book,
    Id,
    Label,
    Message,
    Theme,
    ViewA,
};
use iced::{
    alignment::{Horizontal, Vertical},
    button,
    Alignment,
    Button,
    Checkbox,
    Color,
    Column,
    Command,
    Container,
    Element,
    Length,
    Row,
    Space,
    Text,
    TextInput,
};
use reqwest::Url;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EAdd {
    No,
    Title,
    Missing,
    EmptyClipboard,
}
impl Default for EAdd {
    fn default() -> Self { EAdd::No }
}
impl EAdd {
    fn say(&self) -> &str {
        match self {
            Self::No => "",
            Self::Title => "The book needs a title.",
            Self::Missing => "There's no book.",
            Self::EmptyClipboard => "There's nothing in the clipboard.",
        }
    }
}
pub struct SAdd {
    pub title:      Label,
    pub err:        EAdd,
    pub titleinput: iced::text_input::State,
    pub addbtn:     button::State,
    pub follow:     bool,
    pub read:       bool,
    pub book:       Option<Box<Book>>,
}
impl Default for SAdd {
    fn default() -> Self {
        Self {
            title:      Default::default(),
            err:        Default::default(),
            titleinput: Default::default(),
            addbtn:     Default::default(),
            follow:     true,
            read:       true,
            book:       None,
        }
    }
}
#[derive(Debug, Clone)]
pub enum AAdd {
    AddBook(Box<Book>),
    Fetch(Url),
    UpdateErr(EAdd),
    UpdateTitle(Label),
    UpdateBook(Label, Box<Book>),
    Refresh(Id),
    ToggleFollow(bool),
    ToggleRead(bool),
}

impl SAdd {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn view(
        &mut self, settings: &mut AppSettings, darkmode: bool,
    ) -> Element<'_, Message> {
        let err = Text::new(self.err.say().to_owned())
            .size(22)
            .color(Color {
                r: 200.,
                g: 0.,
                b: 0.,
                a: 0.65,
            })
            .horizontal_alignment(Horizontal::Center)
            .width(Length::Fill);

        let title_input = TextInput::new(
            &mut self.titleinput,
            format!("{: ^1$}", "Title", 9).as_str(),
            self.title.0.as_str(),
            |a| AAdd::UpdateTitle(a.into()).into(),
        )
        .on_submit(
            self.book
                .as_ref()
                .map(|b| AAdd::AddBook(b.to_owned()))
                .unwrap_or(AAdd::UpdateErr(EAdd::Missing))
                .into(),
        )
        .size(32)
        .width(Length::Units(
            (settings.width as u16).min(100.max(self.title.0.len() as u16 * 17)),
        ));

        let follow =
            Checkbox::new(self.follow, "Read", |a| AAdd::ToggleFollow(a).into())
                .style(Theme::from(darkmode))
                .width(Length::Shrink)
                .spacing(8)
                .size(28);

        let read =
            Checkbox::new(self.read, "To list", |a| AAdd::ToggleRead(a).into())
                .style(Theme::from(darkmode))
                .width(Length::Shrink)
                .spacing(4)
                .size(28);

        let addbtn = Button::new(
            &mut self.addbtn,
            Text::new(if self.book.is_none() {
                "Fetch"
            } else {
                " Add "
            })
            .vertical_alignment(Vertical::Center)
            .horizontal_alignment(Horizontal::Center),
        )
        .width(Length::Shrink)
        .height(Length::Units(52))
        .on_press(
            self.book
                .as_ref()
                .map(|b| AAdd::AddBook(b.to_owned()))
                .unwrap_or_else(|| {
                    AAdd::Fetch(
                        "https://zinmanga.com/manga/first-miss-reborn/chapter-2/"
                            .parse()
                            .unwrap(),
                    )
                })
                .into(),
        );
        let title = Column::new()
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .push(err)
            .push(
                Row::new()
                    .align_items(Alignment::Start)
                    .width(Length::Fill)
                    .push(Space::new(Length::Fill, Length::Shrink))
                    .push(title_input)
                    .push(Space::new(Length::Fill, Length::Shrink)),
            );
        let clickables = Row::new()
            .align_items(Alignment::Center)
            .spacing(10)
            .push(
                Column::new()
                    .align_items(Alignment::Start)
                    .push(read)
                    .push(follow),
            )
            .push(addbtn);
        let main = Column::new()
            .align_items(Alignment::Center)
            .spacing(10)
            .push(title)
            .push(clickables);
        Container::new(main)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into()
    }

    pub fn update(
        &mut self, data: &mut AppData, message: AAdd,
    ) -> Command<Message> {
        match message {
            AAdd::Fetch(a) => {
                let r = data.retriever.clone(); // TODO: Check inpact of this clone()
                return Command::perform(
                    async move { r.new_book(a).await },
                    move |(title, book)| AAdd::UpdateBook(title, book).into(),
                );
            }
            AAdd::AddBook(bk) => {
                if self.title.0.is_empty() {
                    return Command::perform(async {}, |_| {
                        AAdd::UpdateErr(EAdd::Title).into()
                    });
                }
                data.library.add_book(&self.title, *bk);
                self.book = None;
                if self.read || self.follow {
                    let id = *data.library.titles.id(&self.title).unwrap();
                    if self.read {
                        data.library.add_to_group("Reading", id)
                    }
                    let mut cmds = vec![];
                    if self.follow {
                        cmds.push(Command::perform(async {}, |_| {
                            AppState::Reader.into()
                        }));
                        cmds.push(Command::perform(async {}, move |_| {
                            ALib::Select(id).into()
                        }))
                    }
                    cmds.push(Command::perform(async {}, move |_| {
                        AAdd::UpdateTitle("".into()).into()
                    }));
                    return Command::batch(cmds);
                }
            }
            AAdd::UpdateErr(t) => {
                self.err = t;
                if !matches!(self.err, EAdd::No) {
                    return Command::perform(
                        async {
                            sleep(Duration::from_secs_f32(4.)).await;
                        },
                        |_| AAdd::UpdateErr(EAdd::No).into(),
                    );
                }
            }
            AAdd::UpdateTitle(t) => {
                self.title = t;
                matches!(self.err, EAdd::Title).then(|| {
                    self.err = EAdd::No;
                });
            }
            AAdd::UpdateBook(t, b) => {
                self.title = t;
                self.book = Some(b);
            }
            AAdd::Refresh(_) => todo!(),
            AAdd::ToggleFollow(b) => self.follow = b,
            AAdd::ToggleRead(b) => self.read = b,
        }
        Command::none()
    }
}
impl From<AAdd> for Message {
    fn from(a: AAdd) -> Self { Message::Update(ViewA::AAdd(a)) }
}
impl From<EAdd> for Message {
    fn from(a: EAdd) -> Self { Message::Update(ViewA::AAdd(AAdd::UpdateErr(a))) }
}
