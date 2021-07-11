use crate::{settings::AppState, AppData, AppSettings, Id, Message};
use iced::{Command, Element};

pub mod sadd;
pub mod slib;
pub mod sread;
pub mod sset;

pub use self::{sadd::*, slib::*, sread::*, sset::*};

#[derive(Debug)]
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
    Select(Id),
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
        &'a mut self, data: &'a mut AppData, settings: &AppSettings,
    ) -> Element<Message> {
        match self.state {
            AppState::Settings => self.sset.view(),
            AppState::Reader => self.sread.view(data, settings),
            AppState::Library => self.slib.view(),
            AppState::Add => self.sadd.view(data),
        }
    }

    pub fn update(
        &mut self, data: &mut AppData, _settings: &AppSettings, message: ViewA,
    ) -> Command<Message> {
        match &message {
            ViewA::ASet(_) => todo!(),
            ViewA::ARead(_) => todo!(),
            ViewA::ALib(ALib::Select(_id)) => {
                self.state = AppState::Reader;
            }
            ViewA::AAdd(AAdd::Add(_url)) => {
                self.state = AppState::Reader;
            }
            _ => (),
        }
        match message {
            ViewA::Select(num) => {
                data.current = Box::new(
                    data.library
                        .book_by_id(num)
                        .current()
                        .map(|(_, v)| v.clone())
                        .take(self.sread.per as usize)
                        .collect(),
                );
            }
            ViewA::Switch(s) => self.state = s,
            ViewA::ASet(a) => {
                self.sset.update(a);
            }
            ViewA::ARead(a) => {
                self.sread.update(a);
            }
            ViewA::ALib(a) => {
                self.slib.update(a);
            }
            ViewA::AAdd(_a) => {
                // Command::from_future(self.sadd.update(data,a), a);
            }
        };
        Command::none()
    }
}
