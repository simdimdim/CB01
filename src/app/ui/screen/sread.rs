use crate::{data::AppData, AppSettings, Book, Content, Label, Message, ViewA};
use iced::{
    scrollable,
    Align,
    Command,
    Container,
    Element,
    Length,
    Row,
    Scrollable,
};
use itertools::Either;
use std::{path::PathBuf, rc::Rc};

#[derive(Debug)]
pub struct SRead {
    pub scroff: f32,
    pub scroll: scrollable::State,
    pub per:    u16,
    pub book:   Option<Label>,
}
#[derive(Debug, Clone, Copy)]
pub enum ARead {
    Scrolled(f32),
    Begin,
    Prev(f32),
    Next(f32),
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
        }
    }

    pub fn view<'a>(
        &'a mut self, data: &'a mut AppData, settings: &AppSettings,
    ) -> Element<'a, Message> {
        let re = data.reversed;
        let col = self.per;
        let res = if re {
            Either::Left(data.library.current().current().rev())
        } else {
            Either::Right(data.library.current().current())
        }
        .take(self.per as usize)
        .fold(
            Row::new().align_items(Align::Center),
            |mut row, (_, cnt)| {
                let el = cnt.view(Some(col));
                row = row.push(el);
                row
            },
        );
        let scroll = Scrollable::new(&mut self.scroll)
            .align_items(Align::Center)
            .on_scroll(move |off| ARead::Scrolled(off).into())
            .push(res.max_width(settings.width).max_height(settings.height))
            .max_width(settings.width);
        // TODO: skip n take, chunk
        //        let _cn = data.current.chunks_mut(self.per.max(1) as
        // usize).fold(          Scrollable::new(&mut self.scroll)
        //             .align_items(Align::Center)
        //           .on_scroll(move |off| ARead::Scrolled(off).into()),
        //            |mut content, ch| {
        //                if re {
        //                    ch.reverse();
        //                }
        //                content = content
        //                    .push(ch.into_iter().fold(
        //                        Row::new().align_items(Align::Center),
        //                        |mut row, cnt| {
        //                            let elem = cnt.view(Some(col));
        //                            row = row
        //                                .push(elem)
        //                                .max_width(settings.width)
        //                                .max_height(settings.height);
        //                            row
        //                        },
        //                    ))
        //                    .max_width(settings.width);
        //                content
        //            },
        //        );
        if re {
            data.reversed = !data.reversed;
        }
        Container::new(scroll)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Align::Center)
            .align_y(Align::Center)
            .into()
    }

    pub fn update(&mut self, message: ARead) -> Command<Message> {
        match message {
            ARead::Scrolled(off) => {
                self.scroff = off;
            }
            ARead::Begin => {
                self.scroff = 0.;
                self.scroll.snap_to(self.scroff);
            }
            ARead::Prev(current) => {
                self.scroff = (self.scroff -
                    (current / self.per as f32 - 1.).recip())
                .max(0.);
                self.scroll.snap_to(self.scroff);
            }
            ARead::Next(current) => {
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
            }
            ARead::Less => {
                self.per = 1.max(self.per.saturating_sub(1));
            }
        }
        Command::none()
    }
}
impl From<ARead> for Message {
    fn from(a: ARead) -> Self { Message::Update(ViewA::ARead(a)) }
}
