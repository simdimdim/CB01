use crate::{settings::AppState, AppData, AppSettings, Message};
use iced::{Command, Element};

pub mod sadd;
pub mod sbook;
pub mod slib;
pub mod sread;
pub mod sset;

pub use self::{sadd::*, sbook::*, slib::*, sread::*, sset::*};

pub struct Screens {
    pub sset:  SSet,
    pub slib:  SLib,
    pub sread: SRead,
    pub sadd:  SAdd,
    pub sbook: SBook,
    pub state: AppState,
}
#[derive(Debug, Clone)]
pub enum ViewA {
    Switch(AppState),
    ASet(ASet),
    ARead(ARead),
    ALib(ALib),
    AAdd(AAdd),
    ABook(ABook),
}

impl<'a> Screens {
    pub fn new() -> Self {
        Self {
            sset:  SSet::new(),
            slib:  SLib::new(),
            sread: SRead::new(),
            sadd:  SAdd::new(),
            sbook: SBook::new(),
            state: AppState::Info,
        }
    }

    pub fn view(
        &'a mut self, data: &'a mut AppData, settings: &'a mut AppSettings,
    ) -> Element<'_, Message> {
        match self.state {
            AppState::Settings => self.sset.view(settings),
            AppState::Reader => {
                self.sread.view(data, settings, self.sset.darkmode)
            }
            AppState::Library => self.slib.view(data),
            AppState::Add => self.sadd.view(settings, self.sset.darkmode),
            AppState::Info => self.sbook.view(data, self.sset.darkmode),
        }
    }

    pub fn update(
        &mut self, data: &mut AppData, settings: &AppSettings, message: ViewA,
    ) -> Command<Message> {
        match message {
            ViewA::ALib(ALib::Select(id)) => {
                self.sread.blabel = data.library.titles.title(id);
                self.state = AppState::Reader;
                return self.slib.update(ALib::Select(id));
            }
            ViewA::Switch(s) => self.state = s,
            ViewA::ASet(a) => return self.sset.update(a),
            ViewA::ARead(a) => return self.sread.update(data, settings, a),
            ViewA::ALib(a) => return self.slib.update(a),
            ViewA::AAdd(a) => return self.sadd.update(data, a),
            ViewA::ABook(a) => return self.sbook.update(data, a),
        };
        Command::none()
    }
}
