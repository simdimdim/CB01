use crate::{data::AppData, AppSettings, Label, Message, ViewA};
use iced::{
    scrollable,
    Align,
    Command,
    Container,
    Element,
    Length,
    Row,
    Scrollable,
    Space,
};
use itertools::Either;

#[derive(Debug)]
pub struct SRead {
    pub scroff: f32,
    pub scroll: scrollable::State,
    pub per:    u16,
    pub book:   Option<Label>,
    pub id:     Option<u16>,
    pub single: bool,
    pub rev:    bool,
    pub flip:   bool,
}
#[derive(Debug, Clone, Copy)]
pub enum ARead {
    Scrolled(f32),
    Begin,
    Prev,
    Next,
    End,
    More,
    Less,
}

impl SRead {
    pub fn new() -> Self {
        Self {
            scroff: 0f32,
            scroll: scrollable::State::new(),
            per:    1,
            book:   None,
            id:     None,
            single: false,
            rev:    false,
            flip:   false,
        }
    }

    pub fn view<'a>(
        &'a mut self, data: &'a mut AppData, settings: &AppSettings,
    ) -> Element<'a, Message> {
        let pics = data.library.book(self.book.as_ref().unwrap()).current();
        let res = if !self.single {
            Either::Left(
                if self.rev {
                    Either::Left(pics.rev())
                } else {
                    Either::Right(pics)
                }
                .fold(Row::new(), |mut row, (_n, cnt)| {
                    let el = cnt.view(Some(self.per));
                    row = row.push(el);
                    row
                }),
            )
        } else {
            let mut scroll = Scrollable::new(&mut self.scroll);
            for (_, cnt) in pics {
                let el = cnt.view(Some(self.per));
                scroll = scroll.push(el);
            }
            let row = Row::new()
                .push(Space::new(Length::Fill, Length::Fill))
                .push(
                    scroll
                        .on_scroll(move |off| ARead::Scrolled(off).into())
                        .width(Length::FillPortion(2))
                        .max_width(settings.width),
                )
                .push(Space::new(Length::Fill, Length::Fill));
            Either::Right(row)
        };

        match res {
            Either::Left(res) => Container::new(
                res.max_width(settings.width)
                    .max_height(settings.height)
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .align_items(Align::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Align::Center)
            .align_y(Align::Center)
            .into(),
            Either::Right(scroll) => Container::new(scroll)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Align::Center)
                .align_y(Align::Center)
                .into(),
        }
    }

    pub fn update(
        &mut self, data: &mut AppData, message: ARead,
    ) -> Command<Message> {
        match message {
            ARead::Scrolled(off) => {
                self.scroff = off;
            }
            ARead::Begin => {
                self.scroff = 0.;
                self.scroll.snap_to(self.scroff);
            }
            ARead::Prev => {
                // TODO: Add a bit more logic concerning single strip
                // and multi-page modes
                let mut current = 0.;
                self.book.as_ref().map(|t| {
                    let book = data.library.book_mut(t);
                    book.backtrack_by(self.per);
                    current = book.last().len as f32;
                });
                self.scroff = (self.scroff -
                    (current / self.per as f32 - 1.).recip())
                .max(0.);
                self.scroll.snap_to(self.scroff);
            }
            ARead::Next => {
                // TODO: Add a bit more logic concerning single strip
                // and multi-page modes
                let mut current = 0.;
                self.book.as_ref().map(|t| {
                    let book = data.library.book_mut(t);
                    book.advance_by(self.per);
                    current = book.last().len as f32;
                });
                self.scroff = (self.scroff +
                    (current / self.per as f32 - 1.).recip())
                .min(1.);
                self.scroll.snap_to(self.scroff);
            }
            ARead::End => {
                self.scroff = 1.;
                self.scroll.snap_to(self.scroff);
            }
            ARead::More => {
                self.per = self.per.saturating_add(1);
                self.book.as_ref().map(|t| {
                    data.library
                        .book_mut(t)
                        .chap_set_len(0, Some(self.per))
                        .current();
                });
            }
            ARead::Less => {
                self.per = 1.max(self.per.saturating_sub(1));
                self.book.as_ref().map(|t| {
                    data.library
                        .book_mut(t)
                        .chap_set_len(0, Some(self.per))
                        .current();
                });
            }
        }
        Command::none()
    }
}
impl From<ARead> for Message {
    fn from(a: ARead) -> Self { Message::Update(ViewA::ARead(a)) }
}
