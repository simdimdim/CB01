use crate::{settings::AppState, AppData, AppSettings, Id, Message};
use iced::{Command, Element};

pub mod sadd;
pub mod slib;
pub mod sread;
pub mod sset;

pub use self::{sadd::*, slib::*, sread::*, sset::*};

pub struct Screens {
    pub sset:  SSet,
    pub slib:  SLib,
    pub sread: SRead,
    pub sadd:  SAdd,
    pub state: AppState,
}
#[derive(Debug, Clone)]
pub enum ViewA {
    Switch(AppState),
    ASet(ASet),
    ARead(ARead),
    ALib(ALib),
    AAdd(AAdd),
}

impl<'a> Screens {
    pub fn new() -> Self {
        Self {
            sset:  SSet::new(),
            slib:  SLib::new(),
            sread: SRead::new(),
            sadd:  SAdd::new(),
            state: AppState::Library,
        }
    }

    pub fn view(
        &'a mut self, data: &'a mut AppData, settings: &'a mut AppSettings,
    ) -> Element<Message> {
        match self.state {
            AppState::Settings => self.sset.view(settings),
            AppState::Reader => self.sread.view(data, settings),
            AppState::Library => self.slib.view(data),
            AppState::Add => self.sadd.view(),
        }
    }

    pub fn update(
        &mut self, data: &mut AppData, _settings: &AppSettings, message: ViewA,
    ) -> Command<Message> {
        match message {
            ViewA::ARead(ARead::Next(off)) => {
                let cur = data.library.current();
                cur.advance_by(self.sread.per);
                return self.sread.update(ARead::Next(off));
            }
            ViewA::ARead(ARead::Prev(off)) => {
                let cur = data.library.current();
                cur.backtrack_by(self.sread.per);
                return self.sread.update(ARead::Prev(off));
            }
            ViewA::ALib(ALib::Select(id)) => {
                data.current = Box::new(
                    data.library
                        .book_by_id(id)
                        .current()
                        .map(|(_, v)| v.clone())
                        .take(self.sread.per as usize)
                        .collect(),
                );
                self.state = AppState::Reader;
                return self.slib.update(ALib::Select(id));
            }
            ViewA::Switch(s) => self.state = s,
            ViewA::ASet(a) => return self.sset.update(a),
            ViewA::ARead(a) => return self.sread.update(a),
            ViewA::ALib(a) => return self.slib.update(a),
            ViewA::AAdd(a) => return self.sadd.update(data, a),
        };
        Command::none()
    }
}
