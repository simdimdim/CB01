use crate::{data::AppData, AppSettings, Content, Label, Message, ViewA};
use iced::{
    alignment::{Horizontal, Vertical},
    scrollable,
    Alignment,
    Command,
    Container,
    Element,
    Length,
    Row,
    Scrollable,
    Space,
};
use itertools::Either;
use log::trace;

#[derive(Debug)]
pub struct SRead {
    pub scroll: scrollable::State,
    pub scroff: f32,
    pub per:    u16,
    pub blabel: Option<Label>,
    pub id:     Option<u16>,
    pub single: bool,
    pub smooth: bool,
    pub rev:    bool,
    pub flip:   bool,
    pub switch: bool,
}
#[derive(Debug, Clone, Copy)]
pub enum ARead {
    Scroll(f32),
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
            scroll: scrollable::State::new(),
            per:    2,
            scroff: 0.,
            blabel: None,
            id:     None,
            single: true,
            smooth: true,
            rev:    false,
            flip:   false,
            switch: false,
        }
    }

    pub fn view<'a>(
        &'a mut self, data: &'a mut AppData, settings: &AppSettings,
        darkmode: bool,
    ) -> Element<'a, Message> {
        let pics = data.library.book(self.blabel.as_ref().unwrap()).current();
        if !self.single {
            let res = if self.rev {
                Either::Left(pics.rev())
            } else {
                Either::Right(pics)
            }
            .fold(Row::new(), |mut row, (_n, cnt)| {
                let el = cnt.view((self.per > 1).then(|| self.per), darkmode);
                row = row.push(el);
                row
            });
            return Container::new(
                res.max_width(settings.width)
                    .max_height(settings.height)
                    .width(Length::Shrink)
                    .height(Length::Shrink)
                    .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .into();
        } else {
            let mut scroll = Scrollable::new(&mut self.scroll)
                .on_scroll(move |off| {
                    if off >= 1. {
                        ARead::Next.into()
                    } else if off == 0.0f32 {
                        ARead::Prev.into()
                    } else {
                        ARead::Scroll(off).into()
                    }
                })
                .max_width(settings.width);
            for (_, cnt) in pics {
                if let content @ (Content::Image { .. } | Content::Text { .. }) =
                    cnt
                {
                    let el = content.view(Some(2), darkmode);
                    let row = Row::new()
                        .push(Space::new(Length::Fill, Length::Shrink))
                        .push(el)
                        .push(Space::new(Length::Fill, Length::Shrink))
                        .height(Length::Shrink);
                    scroll = scroll.push(row);
                }
            }
            return Container::new(scroll)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .into();
        };
    }

    pub fn update(
        &mut self, data: &mut AppData, settings: &AppSettings, message: ARead,
    ) -> Command<Message> {
        match message {
            ARead::Scroll(off) => {
                trace!("Scrolled: {}", off);
                self.scroff = off;
            }
            ARead::Next => {
                if let Some(book) =
                    self.blabel.as_ref().map(|t| data.library.book_mut(t))
                {
                    if self.scroff >= 1. || !self.single {
                        book.advance_by(self.per);
                        self.update(data, settings, ARead::Begin);
                    } else {
                        let to = (self.scroff + (self.per as f32 * 1.5).recip())
                            .min(1.);
                        self.scroff = to;
                        self.scroll.snap_to(to);
                    }
                }
            }
            ARead::Prev => {
                if let Some(book) =
                    self.blabel.as_ref().map(|t| data.library.book_mut(t))
                {
                    if self.scroff <= f32::EPSILON || !self.single {
                        book.backtrack_by(self.per);
                        self.update(
                            data,
                            settings,
                            if self.smooth {
                                ARead::Begin
                            } else {
                                ARead::End
                            },
                        );
                    } else {
                        let to = (self.scroff - (self.per as f32 * 1.5).recip())
                            .max(0.);
                        self.scroff = to;
                        self.scroll.snap_to(to);
                    }
                }
            }
            ARead::Begin => {
                self.scroff = 0.;
                self.scroll.snap_to(0.);
            }
            ARead::End => {
                self.scroff = 1.;
                self.scroll.snap_to(1.);
            }
            ARead::More => {
                self.per = self.per.saturating_add(1);
                if let Some(book) =
                    self.blabel.as_ref().map(|t| data.library.book_mut(t))
                {
                    book.chap_set_len(0, Some(self.per)).current();
                    trace!("{:?}", book.last());
                }
            }
            ARead::Less => {
                self.per = 1.max(self.per.saturating_sub(1));
                if let Some(book) =
                    self.blabel.as_ref().map(|t| data.library.book_mut(t))
                {
                    book.chap_set_len(0, Some(self.per)).current();
                    trace!("{:?}", book.last());
                }
            }
        }
        Command::none()
    }
}
impl From<ARead> for Message {
    fn from(a: ARead) -> Self { Message::Update(ViewA::ARead(a)) }
}
// fn f(off: f32, book: Book) {
//     if off >= 1. && !book.is_last() {
//         return Command::batch(vec![
//             Command::perform(async {}, |_| ARead::Next.into()),
//             Command::perform(async {}, |_| ARead::Scroll(0.).into()),
//         ]);
//     }
//     if off <= 0. && book.last().offset != 0 {
//         let smooth = (self.smooth as u8) as f32;
//         return Command::batch(vec![
//             Command::perform(async {}, |_| ARead::Prev.into()),
//             Command::perform(async {}, move |_| ARead::Scroll(smooth).into()),
//         ]);
//     }
// }
