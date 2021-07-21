use crate::{data::AppData, AppSettings, Content, Label, Message, ViewA};
use iced::{
    scrollable,
    Align,
    Command,
    Container,
    Element,
    Length,
    Rectangle,
    Row,
    Scrollable,
    Space,
};
use itertools::Either;
use log::{error, info, warn};

#[derive(Debug)]
pub struct SRead {
    pub scroll: scrollable::State,
    pub per:    u16,
    pub blabel: Option<Label>,
    pub id:     Option<u16>,
    pub single: bool,
    pub rev:    bool,
    pub flip:   bool,
}
#[derive(Debug, Clone, Copy)]
pub enum ARead {
    Scroll(f32, bool),
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
            blabel: None,
            id:     None,
            single: true,
            rev:    false,
            flip:   false,
        }
    }

    pub fn view<'a>(
        &'a mut self, data: &'a mut AppData, settings: &AppSettings,
        darkmode: bool,
    ) -> Element<'a, Message> {
        let pics = data.library.book(self.blabel.as_ref().unwrap()).current();
        let res = if !self.single {
            Either::Left(
                if self.rev {
                    Either::Left(pics.rev())
                } else {
                    Either::Right(pics)
                }
                .fold(Row::new(), |mut row, (_n, cnt)| {
                    let el = cnt.view((self.per > 1).then(|| self.per), darkmode);
                    row = row.push(el);
                    row
                }),
            )
        } else {
            let mut scroll = Scrollable::new(&mut self.scroll);
            for (_, content) in pics {
                if let content @ Content::Image { .. } |
                content @ Content::Text { .. } = content
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
            Either::Right(
                scroll
                    .on_scroll(move |off| ARead::Scroll(off, true).into())
                    .max_width(settings.width),
            )
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
        &mut self, data: &mut AppData, settings: &AppSettings, message: ARead,
    ) -> Command<Message> {
        match message {
            ARead::Scroll(off, should_call) => {
                if let Some(t) = self.blabel.as_ref() {
                    let book = data.library.book(t);
                    if !should_call {
                        self.scroll.scroll(
                            off,
                            Rectangle {
                                x:      0.,
                                y:      0.,
                                width:  settings.width as f32,
                                height: settings.height as f32,
                            },
                            Rectangle {
                                x:      0.,
                                y:      0.,
                                width:  settings.width as f32,
                                height: settings.height as f32,
                            },
                        );
                        return Command::none();
                    }
                    if off >= 1. && !book.is_last() && should_call {
                        return Command::batch(vec![
                            Command::perform(async {}, |_| ARead::Next.into()),
                            Command::perform(async {}, move |_| {
                                ARead::Scroll(0., false).into()
                            }),
                        ]);
                    }
                    if off <= 0. && book.last().offset != 0 && should_call {
                        return Command::batch(vec![
                            Command::perform(async {}, |_| ARead::Prev.into()),
                            Command::perform(async {}, move |_| {
                                ARead::Scroll(1., false).into()
                            }),
                        ]);
                    }
                    warn!("Scrolled to: {:.4}", &off);
                }
            }
            ARead::Next => {
                // TODO: Add a bit more logic concerning single strip
                // and multi-page modes
                match self.single {
                    false => {
                        if let Some(t) = self.blabel.as_ref() {
                            data.library.book_mut(t).advance_by(self.per);
                            info!("{:?}", data.library.book(t).last());
                        }
                    }
                    true => {
                        if let Some(t) = self.blabel.as_ref() {
                            let book = data.library.book_mut(t);
                            book.advance_by(self.per);
                            info!("{:?}", data.library.book(t).last());
                            return Command::perform(async {}, |_| {
                                ARead::Scroll(0., false).into()
                            });
                        }
                    }
                };
            }
            ARead::Prev => {
                // TODO: Add a bit more logic concerning single strip
                // and multi-page modes
                match self.single {
                    false => {
                        if let Some(t) = self.blabel.as_ref() {
                            data.library.book_mut(t).backtrack_by(self.per);
                            info!("{:?}", data.library.book(t).last());
                        }
                    }
                    true => {
                        if let Some(t) = self.blabel.as_ref() {
                            let book = data.library.book_mut(t);
                            book.backtrack_by(self.per);
                            warn!("{:?}", data.library.book(t).last());
                            warn!("{:?}", data.library.book(t).chap_cur());
                            return Command::perform(async {}, |_| {
                                ARead::Scroll(0., false).into()
                            });
                        }
                    }
                };
            }
            ARead::Begin => {
                self.scroll.snap_to(0.);
            }
            ARead::End => {
                self.scroll.snap_to(1.);
            }
            ARead::More => {
                self.per = self.per.saturating_add(1);
                if let Some(t) = self.blabel.as_ref() {
                    data.library
                        .book_mut(t)
                        .chap_set_len(0, Some(self.per))
                        .current();
                    info!("{:?}", data.library.book(t).last());
                }
            }
            ARead::Less => {
                self.per = 1.max(self.per.saturating_sub(1));
                if let Some(t) = self.blabel.as_ref() {
                    data.library
                        .book_mut(t)
                        .chap_set_len(0, Some(self.per))
                        .current();
                    info!("{:?}", data.library.book(t).last());
                }
            }
        }
        Command::none()
    }
}
impl From<ARead> for Message {
    fn from(a: ARead) -> Self { Message::Update(ViewA::ARead(a)) }
}
